use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 示例子命令
    Example {
        /// 示例参数
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Example { name }) => {
            println!("Hello, {}!", name);
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
