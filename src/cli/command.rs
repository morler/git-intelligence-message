use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct GimCli {
    #[command(subcommand)]
    pub command: Option<GimCommands>,

    /// The commit message title
    #[arg(short, long)]
    pub title: Option<String>,

    /// Auto add the changes to the stage
    #[arg(short, long, default_value_t = false)]
    pub auto_add: bool,

    /// Ammend the last commit
    #[arg(short, long, default_value_t = false)]
    pub update: bool,
}

#[derive(Subcommand)]
pub enum GimCommands {
    /// Setup the ai-api configuration
    Ai {
        /// the ai model name
        #[arg(short, long)]
        model: Option<String>,

        /// the ai api key
        #[arg(short = 'k', long)]
        apikey: Option<String>,

        /// the ai api url
        #[arg(short, long)]
        url: Option<String>,

        /// the answer language
        #[arg(short, long)]
        language: Option<String>,
    },
}