use crate::{
    constants::{DIFF_PROMPT, SUBJECT_PROMPT},
    verbose::print_verbose,
};

use super::{
    command::{GimCli, GimCommands},
    http::chat,
};
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
        Some(GimCommands::Ai {
            model,
            apikey,
            url,
            language,
        }) => {
            if model.is_none() && apikey.is_none() && url.is_none() && language.is_none() {
                eprintln!("Error: At least one of ai section parameter must be provided when setup ai section");
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

    let system = DIFF_PROMPT;

    let ai_config = super::ai_configer::get_ai_config();
    if ai_config.is_err() {
        ai_generating_error(
            "Error: ai section is not configured, abort",
            cli.auto_add && changes.len() > 0,
        );
        return;
    }
    let ai_config = match ai_config {
        Ok(config) => config,
        Err(e) => {
            ai_generating_error(
                &format!("Error: Failed to get AI config - {}", e),
                cli.auto_add && changes.len() > 0,
            );
            return;
        }
    };

    let url = match ai_config.get("url").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error(
                "Error: Missing 'url' in AI config",
                cli.auto_add && changes.len() > 0,
            );
            return;
        }
    };
    let model_name = match ai_config.get("model").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error(
                "Error: Missing 'model' in AI config",
                cli.auto_add && changes.len() > 0,
            );
            return;
        }
    };
    let api_key = match ai_config.get("apikey").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error(
                "Error: Missing 'apikey' in AI config",
                cli.auto_add && changes.len() > 0,
            );
            return;
        }
    };
    let language = match ai_config.get("language").and_then(|v| v.as_str()) {
        Some(v) => v,
        None => {
            ai_generating_error(
                "Error: Missing 'language' in AI config",
                cli.auto_add && changes.len() > 0,
            );
            return;
        }
    };

    if language != "English" {
        diff_content.push_str(&format!(
            "\n The answer should be in {} language. If you cannot recognize this language, use English instead.",
            language
        ));
    }
    let res = chat(
        url,
        model_name,
        api_key,
        Some(system),
        &diff_content,
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
        let system = SUBJECT_PROMPT;
        let res = chat(
            url,
            model_name,
            api_key,
            Some(system),
            &format!("The changes are: \n{}", answer),
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

fn ai_generating_error(abort: &str, auto_add: bool) {
    eprintln!("{}", abort);
    if auto_add {
        println!("Noted: your changes are added to git staged area");
    }
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
