use gim_config::directory;
use indoc::indoc;
use std::{fs, io::Result, path::PathBuf};

use crate::constants::{DIFF_PROMPT_FILE, SUBJECT_PROMPT_FILE};

fn file_dirs() -> Result<PathBuf> {
    directory::config_dir()
}

fn trim_diff_prompt() -> String {
    indoc!(r#"
        You are an expert developer specialist in creating git commits.
        Provide a super concise one sentence summary for each changed file, describing the main change made.
        Each line must follow this format {FILE: CHANGES: (CHANGED_LINES_COUNT)}

        Please follow these rules strictly:
        - Output ONLY the lines of summaries, NO explanations, NO markdown, NO code blocks.
        - Each file change gets exactly one line.
        - Do not use general terms like "update" or "change", be specific.
        - Use present tense, active voice, and imperative mood (e.g., "Fix bug" instead of "Fixed bug").
        - Skip project lock files, like 'Cargo.lock' or 'package-lock.json', 'pnpm-lock.yaml', 'yarn.lock'
        - Skip binary files diff content
        - Ignore files under .code folder or .idea folder, unless there aren't other files changed.
        - Avoid phrases like "The main goal is to..." or "Based on...", just state the change directly.
        - The output should be ready to copy-paste as a commit message with no further modification.

        Examples:
        src/main.rs: Add login validation logic (87)
        README.md: Update installation instructions (12)
    "#)
    .to_string()
}

fn trim_subject_prompt() -> String {
    indoc!(r#"
        You are an expert developer specialist in creating git commits messages.
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
        - The number at the end of each file change is the count of changed lines; prioritize summarizing files with more line changes, except for newly added files which have medium priority
        - Output directly only one commit message in plain text with the next format: {type}: {commit_message}.
        - Be as concise as possible, keep the message under 50 characters or letters.
    "#)
    .to_string()
}
/// Returns the diff prompt string, reading from file if available, or using the default if not.
///
/// # Returns
///
/// * `String` containing the diff prompt.
pub fn get_diff_prompt() -> String {
    let trimmed = trim_diff_prompt();
    let path = match file_dirs() {
        Ok(p) => p.join(DIFF_PROMPT_FILE),
        Err(_) => {
            eprintln!("Failed to get config dir for diff prompt");
            return trimmed;
        }
    };

    if !path.exists() {
        if let Err(e) = fs::write(&path, trimmed.clone()) {
            eprintln!("Failed to write diff prompt to file: {}", e);
        }
        return trimmed;
    }

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read diff prompt from file: {}", e);
        trimmed
    })
}

/// Returns the subject prompt string, reading from file if available, or using the default if not.
///
/// # Returns
///
/// * `String` containing the subject prompt.
pub fn get_subject_prompt() -> String {
    let trimmed = trim_subject_prompt();
    let path = match file_dirs() {
        Ok(p) => p.join(SUBJECT_PROMPT_FILE),
        Err(_) => {
            eprintln!("Failed to get config dir for subject prompt");
            return trimmed;
        }
    };

    if !path.exists() {
        if let Err(e) = fs::write(&path, trimmed.clone()) {
            eprintln!("Failed to write subject prompt to file: {}", e);
        }
        return trimmed;
    }

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read subject prompt from file: {}", e);
        trimmed
    })
}
