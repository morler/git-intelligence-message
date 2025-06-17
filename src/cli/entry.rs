use crate::{
    cli::{http::get_url_by_model, prompt::{get_diff_prompt, get_subject_prompt, DIFF_PROMPT_FILE, SUBJECT_PROMPT_FILE}},
    verbose::print_verbose,
};

use super::{
    command::{GimCli, GimCommands},
    http::chat,
};
use gim_config::directory;
use std::process::Command;

pub async fn run_cli(cli: &GimCli, mut config: toml::Value) {
    match &cli.command {
        Some(GimCommands::Update { force }) => {
            if let Err(e) = crate::cli::update::check_and_install_update(*force).await {
                eprintln!("Failed to update: {}", e);
                std::process::exit(1);
            }
            return;
        }
        Some(GimCommands::Prompt {
            edit,
            prompt,
            editor,
        }) => {
            if let Err(e) = handle_prompt_command(*edit, prompt.as_deref(), editor.as_deref()) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            return;
        }
        Some(GimCommands::Ai {
            model,
            apikey,
            url,
            language,
        }) => {
            if model.is_none() && apikey.is_none() && url.is_none() && language.is_none() {
                let ai = get_validated_ai_config(false, false);
                if let Some(ai) = ai {
                    let mut url = ai.0;
                    if url.is_empty() && !ai.1.is_empty() {
                        if let Some(str) = get_url_by_model(&ai.1) {
                            url = format!("(not configured. Will use default : {})", str);
                        } else {
                            eprintln!("Warning: you have not setup api url by 'gim ai -u <url>'");
                        }
                    }
                    println!(
                        r#"Model:      {}
API Key:    {}
URL:        {}
Language:   {}
You can use 'gim ai -m <model> -k <apikey> -u <url> -l <language>' respectively to update the configuration"#,
                        &ai.1, &ai.2, &url, &ai.3
                    );
                } else {
                    eprintln!("Error: ai section is not configured");
                }
                return;
            }
            super::ai_configer::update_ai_config(&mut config, model, apikey, url, language);
            return;
        }
        None => {}
    }

    // Check if current directory is a git repository
    // git rev-parse --is-inside-work-tree
    let is_git_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output();
    if is_git_repo.is_err() || !is_git_repo.unwrap().status.success() {
        eprintln!("Error: should run in a git repository");
        return;
    }

    // Get git status
    // git status -s --untracked-files=no
    let status_output = Command::new("git")
        .args([
            "status",
            "-s",
            &format!(
                "--untracked-files={}",
                if cli.auto_add { "all" } else { "no" }
            ),
        ])
        .output()
        .expect("Failed to get git status");
    print_verbose(&format!(
        "Run 'git status -s --untracked-files={}'",
        if cli.auto_add { "all" } else { "no" }
    ));
    let status_str = String::from_utf8_lossy(&status_output.stdout);
    let changes: Vec<&str> = status_str.lines().collect();
    let mut diff_content = String::new();
    if !changes.is_empty() {
        println!("Found {} changes:", changes.len());
        for entry in changes.iter() {
            println!(
                "{:?} {}",
                entry,
                if !cli.auto_add && (entry.starts_with(' ') || entry.starts_with('?')) {
                    " - <<Ignored>>"
                } else {
                    ""
                }
            );
        }

        // Auto add changes if enabled
        if cli.auto_add {
            let add_output = Command::new("git")
                .args(["add", "."])
                .output()
                .expect("Failed to execute git add");
            if !add_output.status.success() {
                eprintln!("Error: Failed to add changes to git");
                return;
            }
            print_verbose("Run 'git add .'");
        }

        // Get staged changes
        let diff_output = Command::new("git")
            .args(["diff", "--cached"])
            .output()
            .expect("Failed to get git diff --cached");
        print_verbose("Run 'git diff --cached'");
        if !diff_output.stdout.is_empty() {
            diff_content.push_str("When I use `git diff`, I got the following output: \n");
            diff_content.push_str(&String::from_utf8_lossy(&diff_output.stdout));
            diff_content.push_str("\n");
        }
    }
    if cli.update {
        diff_content.push_str(
            "As I want to amend commit message, I use `git show` and got the following output: \n",
        );

        // Get last commit changes
        let show_output = Command::new("git")
            .args(["show", "--pretty=format:", "HEAD"])
            .output()
            .expect("Failed to get git show");
        print_verbose("Run 'git show --pretty=format: HEAD'");
        diff_content.push_str(&String::from_utf8_lossy(&show_output.stdout));
        diff_content.push_str("\n");
        println!("As '-p' option is enabled, I will amend the last commit message");
    }
    if diff_content.is_empty() {
        println!("No changes found. To update last commit message, please use '-p' option");
        return;
    }

    let config_result = get_validated_ai_config(cli.auto_add, changes.len() > 0);
    if config_result.is_none() {
        return;
    }
    let (url, model_name, api_key, language) = config_result.unwrap();

    if language != "English" {
        diff_content.push_str(&format!(
            "\n The answer should be in {} language. If you cannot recognize this language, use English instead.",
            language
        ));
    }
    let system = get_diff_prompt();
    let res = chat(
        url.clone(),
        model_name.clone(),
        api_key.clone(),
        Some(system),
        diff_content.clone(),
        cli.verbose,
    )
    .await;
    if let Err(e) = res {
        ai_generating_error(&format!("Error: {}", e), cli.auto_add && changes.len() > 0);
        return;
    }
    let answer = res.unwrap();

    let mut commit_subject = cli.title.clone();
    if commit_subject.is_none() {
        let system = get_subject_prompt();
        let res = chat(
            url,
            model_name,
            api_key,
            Some(system),
            format!("The changes are: \n{}", answer),
            cli.verbose,
        )
        .await;

        match res {
            Ok(answer) => {
                commit_subject = Some(answer);
            }
            Err(e) => {
                commit_subject = Some(format!("Error: {}", e));
            }
        }
    }
    let commit_subject = commit_subject.unwrap();
    print_verbose(&format!("AI chat content: {}", diff_content));
    println!();
    println!(
        r#"
>>>>>>>>>>>>>>>>>>>>>>>>>
Commit subject: "{}""#,
        commit_subject
    );
    println!(
        r#"
Commit message: "{}"
<<<<<<<<<<<<<<<<<<<<<<<<<
"#,
        answer
    );

    // Prepare commit message
    let mut commit_args = vec!["commit"];
    if cli.update {
        commit_args.push("--amend");
    }
    commit_args.extend(["-m", &commit_subject, "-m", &answer]);

    // Execute git commit
    print_verbose("Run 'git commit -m <subject> -m <message>'");
    let commit_output = Command::new("git")
        .args(&commit_args)
        .output()
        .expect("Failed to execute git commit");

    if commit_output.status.success() {
        println!("✅ Successfully committed changes! If you were discontent with the commit message and want to polish or revise it, run 'gim -p' or 'git commit --amend'");
    } else {
        eprintln!(
            "Error: Failed to commit changes - {}",
            String::from_utf8_lossy(&commit_output.stderr)
        );
    }
}

fn handle_prompt_command(
    edit: bool,
    prompt: Option<&str>,
    editor: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = directory::config_dir()?;
    let diff_prompt_path = config_dir.join(DIFF_PROMPT_FILE);
    let subject_prompt_path = config_dir.join(SUBJECT_PROMPT_FILE);

    let diff_prompt = get_diff_prompt();
    let subject_prompt = get_subject_prompt();

    if edit {
        if let Some(prompt_type) = prompt {
            let file_path = match prompt_type.to_lowercase().as_str() {
                "d" | "diff" | "diff_prompt" | DIFF_PROMPT_FILE => diff_prompt_path,
                "s" | "subject" | "subject_prompt" | SUBJECT_PROMPT_FILE => subject_prompt_path,
                _ => {
                    return Err(format!(
                        "Unknown prompt type '{}'. Use 'd' or 'diff' or 'diff_prompt' for diff prompt, and 's' or 'subject' or 'subject_prompt' for subject prompt",
                        prompt_type
                    )
                    .into())
                }
            };

            if let Some(editor) = editor {
                // Use the specified editor
                if let Err(e) = Command::new(editor).arg(&file_path).status() {
                    eprintln!("Failed to open file with editor '{}': {}", editor, e);
                }
            } else {
                // Open the directory with default file manager
                if cfg!(target_os = "macos") {
                    Command::new("open")
                        .arg("-R") // Reveal in Finder
                        .arg(&file_path)
                        .status()?;
                } else if cfg!(target_os = "windows") {
                    Command::new("explorer")
                        .arg("/select,")
                        .arg(&file_path)
                        .status()?;
                } else {
                    // Linux and others
                    if let Err(_) = Command::new("xdg-open")
                        .arg(file_path.parent().unwrap_or_else(|| ".".as_ref()))
                        .status()
                    {
                        return Err(
                            "Failed to open file manager. Please specify an editor with --editor"
                                .into(),
                        );
                    }
                }
            }
        } else {
            // Open the directory with default file manager
            if cfg!(target_os = "macos") {
                Command::new("open")
                    // .arg("-R") // Reveal in Finder
                    .arg(&config_dir)
                    .status()?;
            } else if cfg!(target_os = "windows") {
                Command::new("explorer").arg(&config_dir).status()?;
            } else {
                // Linux and others
                Command::new("xdg-open").arg(&config_dir).status()?;
            }
            println!(
                r#"
Please edit the prompt files using your favorite editor in the popped window: {}
1: {}
2: {}"#,
                config_dir.display(),
                DIFF_PROMPT_FILE,
                SUBJECT_PROMPT_FILE
            );
        }
    } else {
        // Show the content of both prompt files
        println!(
            r#"
=== Diff Prompt ===
{}

=== Subject Prompt ===
{}"#,
            &diff_prompt, &subject_prompt
        );
    }

    Ok(())
}

fn ai_generating_error(abort: &str, auto_add: bool) {
    eprintln!("{}", abort);
    if auto_add {
        println!("Noted: your changes are added to git staged area");
    }
}

fn get_validated_ai_config(
    auto_add: bool,
    changed: bool,
) -> Option<(String, String, String, String)> {
    let ai_config = super::ai_configer::get_ai_config();
    if ai_config.is_err() {
        ai_generating_error(
            "Error: ai section is not configured, abort",
            auto_add && changed,
        );
        return None;
    }
    let ai_config = match ai_config {
        Ok(config) => config,
        Err(e) => {
            ai_generating_error(
                &format!("Error: Failed to get AI config - {}", e),
                auto_add && changed,
            );
            return None;
        }
    };

    let url = match ai_config.get("url").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error("Error: Missing 'url' in AI config", auto_add && changed);
            return None;
        }
    };
    let model_name = match ai_config.get("model").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error("Error: Missing 'model' in AI config", auto_add && changed);
            return None;
        }
    };
    let api_key = match ai_config.get("apikey").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error("Error: Missing 'apikey' in AI config", auto_add && changed);
            return None;
        }
    };
    let language = match ai_config.get("language").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error(
                "Error: Missing 'language' in AI config",
                auto_add && changed,
            );
            return None;
        }
    };

    Some((
        url.to_string(),
        model_name.to_string(),
        api_key.to_string(),
        language.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use gim_config::config::get_config;

    use crate::cli::{command::GimCli, entry::run_cli};

    #[tokio::test]
    async fn test_run_cli() {
        let config = get_config().expect("Failed to access config file");
        let cli = GimCli {
            command: None,
            auto_add: false,
            update: true,
            title: None,
            verbose: true,
        };
        run_cli(&cli, config).await;
    }
}
