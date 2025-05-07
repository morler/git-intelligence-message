use clap::{Parser, Subcommand};
use dirs;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// 消息内容
    #[arg(short, long)]
    message: Option<String>,

    /// 自动添加
    #[arg(short, long, default_value_t = false)]
    auto_add: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// set ai model parameters
    Ai {
        /// ai model
        #[arg(short, long)]
        model: String,

        /// ai api key
        #[arg(short, long)]
        apikey: String,

        /// ai api url
        #[arg(short, long)]
        url: String,
    },
}

fn ensure_config_file_exists() -> std::io::Result<PathBuf> {
    let config_dir = dirs::home_dir();
    if config_dir.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ));
    }

    let config_dir = config_dir.unwrap().join(".config/gim/");
    let config_file = config_dir.join("config.toml");
    if !config_file.exists() {
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "config directory not found",
            ));
        }
        let mut settings = toml::map::Map::new();
        settings.insert(
            "example".to_string(),
            toml::Value::String("value".to_string()),
        );
        let mut default_content = toml::map::Map::new();
        default_content.insert("settings".to_string(), toml::Value::Table(settings));
        let default_content = toml::to_string(&toml::Value::Table(default_content))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(&config_file)?;
        file.write_all(default_content.as_bytes())?;
    }
    println!("Config file is {}", config_file.display());
    Ok(config_file)
}

fn run_cli(cli: &Cli, config_file: &PathBuf) {
    let config_content = fs::read_to_string(config_file).expect("Failed to read config file");

    if let Some(Commands::Ai { model, apikey, url }) = &cli.command {
        println!("Hello, {} {} {}", model, apikey, url);
        return;
    }

    if cli.auto_add {
        println!("Auto add mode enabled");
    }

    if let Some(message) = &cli.message {
        println!("Message: {}!", message);
        return;
    }

    println!("No subcommand was used. Config content: {}", config_content);
}

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
