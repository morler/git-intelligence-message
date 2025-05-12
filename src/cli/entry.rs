use super::{
    command::{GimCli, GimCommands},
    http::chat,
};
use std::process::Command;

pub async fn run_cli(cli: &GimCli, mut config: toml::Value) {
    if let Some(GimCommands::Ai {
        model,
        apikey,
        url,
        language,
    }) = &cli.command
    {
        if model.is_none() && apikey.is_none() && url.is_none() && language.is_none() {
            eprintln!("Error: At least one of ai section parameter must be provided when setup ai section");
            return;
        }

        super::handler::update_ai_config(&mut config, model, apikey, url, language);
        return;
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
        }

        diff_content.push_str("When I use `git diff`, I got the following output: \n");

        // Get staged changes
        let diff_output = Command::new("git")
            .args(["diff", "--cached"])
            .output()
            .expect("Failed to get git diff --cached");
        diff_content.push_str(&String::from_utf8_lossy(&diff_output.stdout));
        diff_content.push_str("\n");
    } else if cli.update {
    }
    let system = r#"
    You are an expert developer specialist in creating git commits. 
    Provide a super concise one sentence overall changes summary for each file, following strictly the next rules:
    - Do not use any code snippets, imports, file routes or bullets points.
    - Do not mention the route of file that has been change.
    - Write clear, concise, and descriptive messages that explain the MAIN GOAL made of the changes.
    - Use the present tense and active voice in the message, for example, "Fix bug" instead of "Fixed bug".
    - Use the imperative mood, which gives the message a sense of command, e.g. "Add feature" instead of "Added feature".
    - Avoid using general terms like "update" or "change", be specific about what was updated or changed.
    - Avoid using terms like "The main goal of", just output directly the summary in plain text"#;

    let ai_config = super::handler::get_ai_config();
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

    diff_content.push_str(&format!(
        "\n The answer should be in {} language. If you cannot recognize this language, use English instead.",
        language
    ));
    let res = chat(url, model_name, api_key, Some(system), &diff_content).await;
    if let Err(e) = res {
        ai_generating_error(&format!("Error: {}", e), cli.auto_add && changes.len() > 0);
        return;
    }
    let answer = res.unwrap();

    let mut commit_subject = cli.title.clone();
    if commit_subject.is_none() {
        let system = r#"You are an expert developer specialist in creating git commits messages.
        Your only goal is to retrieve a single commit message.
        Based on the provided user changes, combine them in ONE SINGLE commit message retrieving the global idea, following strictly the next rules:
        - Assign the commit {type} according to the next conditions:
            feat: Only when adding a new feature.
            fix: When fixing a bug.
            docs: When updating documentation.
            style: When changing elements styles or design and/or making changes to the code style (formatting, missing semicolons, etc.) without changing the code logic.
            test: When adding or updating tests.
            chore: When making changes to the build process or auxiliary tools and libraries.
            revert: When undoing a previous commit.
            refactor: When restructuring code without changing its external behavior, or is any of the other refactor types.
        - Do not add any issues numeration, explain your output nor introduce your answer.
        - Output directly only one commit message in plain text with the next format: {type}: {commit_message}.
        - Be as concise as possible, keep the message under 50 characters or letters."#;
        let res = chat(
            url,
            model_name,
            api_key,
            Some(system),
            &format!("The changes are: \n{}", answer),
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
    // println!("Commit body: {}", diff_content);
    println!("Commit subject: {}", commit_subject);
    println!("Commit message: {}", answer);

    // Prepare commit message
    let mut commit_args = vec!["commit"];
    if cli.update {
        commit_args.push("--amend");
    }
    commit_args.extend(["-m", &commit_subject, "-m", &answer]);

    // Execute git commit
    let commit_output = Command::new("git")
        .args(&commit_args)
        .output()
        .expect("Failed to execute git commit");

    if commit_output.status.success() {
        println!("Successfully committed changes!");
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
        println!("Noted: your changes are added to git");
    }
}
