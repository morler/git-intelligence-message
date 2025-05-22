use gim_config::directory;
use std::{fs, io::Result, path::PathBuf};

use crate::constants::{DIFF_PROMPT, SUBJECT_PROMPT};

pub const DIFF_PROMPT_FILE: &str = "diff_prompt.txt";
pub const SUBJECT_PROMPT_FILE: &str = "subject_prompt.txt";

fn file_dirs() -> Result<PathBuf> {
    directory::config_dir()
}

pub fn get_diff_prompt() -> String {
    let path = match file_dirs() {
        Ok(p) => p.join(DIFF_PROMPT_FILE),
        Err(_) => {
            eprintln!("Failed to get config dir for diff prompt");
            return DIFF_PROMPT.to_string();
        }
    };

    if !path.exists() {
        if let Err(e) = fs::write(&path, DIFF_PROMPT) {
            eprintln!("Failed to write diff prompt to file: {}", e);
        }
        return DIFF_PROMPT.to_string();
    }

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read diff prompt from file: {}", e);
        DIFF_PROMPT.to_string()
    })
}

pub fn get_subject_prompt() -> String {
    let path = match file_dirs() {
        Ok(p) => p.join(SUBJECT_PROMPT_FILE),
        Err(_) => {
            eprintln!("Failed to get config dir for subject prompt");
            return SUBJECT_PROMPT.to_string();
        }
    };

    if !path.exists() {
        if let Err(e) = fs::write(&path, SUBJECT_PROMPT) {
            eprintln!("Failed to write subject prompt to file: {}", e);
        }
        return SUBJECT_PROMPT.to_string();
    }

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read subject prompt from file: {}", e);
        SUBJECT_PROMPT.to_string()
    })
}
