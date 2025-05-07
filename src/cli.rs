use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    #[arg(short, long)]
    pub message: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub auto_add: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Ai {
        #[arg(short, long)]
        model: String,
        #[arg(short, long)]
        apikey: String,
        #[arg(short, long)]
        url: String,
    },
}

pub fn run_cli(cli: &Cli, config_file: &PathBuf) {
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
