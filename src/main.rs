use clap::Parser;
use cli::{Cli, run_cli};
use config::ensure_config_file_exists;

mod cli;
mod config;


fn main() {
    let config_file = ensure_config_file_exists().expect("Failed to create or access config file");
    let cli = Cli::parse();
    run_cli(&cli, &config_file);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_ensure_config_file_exists_creates_file() {
        let config_file = ensure_config_file_exists().unwrap();
        assert!(config_file.exists());
        let content = fs::read_to_string(&config_file).unwrap();
        let parsed: toml::Value = toml::from_str(&content).expect("Failed to parse toml");
        let s = parsed.get("settings");
        assert!(s.is_some());
        print!("{:?}", s.unwrap())
    }
}