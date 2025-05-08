use crate::cli::prompts::*;
use git2::*;

use super::command::{Cli, Commands};

pub fn run_cli(cli: &Cli, mut config: toml::Value) {
    if let Some(Commands::Ai {
        model,
        apikey,
        url,
        prompt,
        language,
    }) = &cli.command
    {
        if model.is_none()
            && apikey.is_none()
            && url.is_none()
            && prompt.is_none()
            && language.is_none()
        {
            eprintln!("Error: At least one of --model, --apikey, --url, --prompt or --language must be provided");
            return;
        }

        super::handler::update_ai_config(&mut config, model, apikey, url, prompt, language);
        return;
    }

    // Check if current directory is a git repository
    let repo = Repository::open(".");
    if repo.is_err() {
        eprintln!("Error: should run in a git repository");
        return;
    }
    let repo = repo.unwrap();

    // Check repository status before auto add
    let mut status_opts = StatusOptions::new();
    status_opts
        .include_untracked(true)
        .show(StatusShow::IndexAndWorkdir);
    let changes = repo
        .statuses(Some(&mut status_opts))
        .expect("Failed to get repository status");
    let mut diff_content = String::new();
    let mut unstaged = false;
    if !changes.is_empty() {
        println!("Found {} unstaged changes:", changes.len());
        for entry in changes.iter() {
            if entry.path().is_none() {
                eprintln!("Ignore: unknown file path: {:?}", entry.status());
                continue;
            }
            let path = entry.path().unwrap();
            println!("{:?}  - {}", entry.status(), path);
            if diff_content.len() == 0 {
                diff_content.push_str(GIT_STATUS_PROMPT);
            }
            diff_content.push_str(&format!("{:?}  - {}\n", entry.status(), path));

            if is_wt(&entry.status()) {
                unstaged = true;
            }
        }

        if unstaged && cli.auto_add  {
            let mut index = repo.index().expect("Failed to get repository index");
            index
                .add_all(["."].iter(), IndexAddOption::DEFAULT, None)
                .expect("Failed to add files to index");
            index.write().expect("Failed to write index");
            println!("Successfully added all changes to the staging area: {} files", changes.len());
        }

        for entry in changes.iter() {
            if entry.path().is_none() {
                continue;
            }
            let path = entry.path().unwrap();
            
            // Get diff for staged changes
            let diff = repo.diff_index_to_workdir(
                Some(&repo.index().expect("Failed to get index")),
                Some(&mut DiffOptions::new().pathspec(path))
            ).expect("Failed to get diff");
            
            // Format diff output
            let mut diff_output = String::new();
            diff.print(DiffFormat::Patch, |_, _, line| {
                let line = String::from_utf8(line.content().to_vec());
                if let Ok(line) = line {
                    diff_output.push_str(&line);
                }
                true
            }).expect("Failed to print diff");
            
            if !diff_output.is_empty() {
                diff_content.push_str(&format!("\nChanges for {}:\n{} \n\n", path, diff_output));
            }
        }
    }

    let mut commit_message = cli.message.clone();
    if commit_message.is_none() {}
}

fn is_wt(status: &Status) -> bool {
    status.is_wt_new()
        || status.is_wt_modified()
        || status.is_wt_deleted()
        || status.is_wt_renamed()
        || status.is_wt_typechange()
}
