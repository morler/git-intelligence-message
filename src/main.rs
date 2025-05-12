use cli::{command::GimCli, entry::run_cli};
use config::get_config_into;

mod cli;
mod config;

#[tokio::main]
async fn main() {
    let config = get_config_into().expect("Failed to access config file");
    let cli = <GimCli as clap::Parser>::parse();
    run_cli(&cli, config).await;
}
