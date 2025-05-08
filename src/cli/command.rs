use clap::{Parser, Subcommand};

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
        model: Option<String>,
        #[arg(short, long)]
        apikey: Option<String>,
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        prompt: Option<String>,
        #[arg(short, long)]
        language: Option<String>,
    },
}