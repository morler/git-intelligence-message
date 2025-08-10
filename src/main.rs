use cli::{command::GimCli, entry::run_cli, update::check_update_reminder, verbose};
use gim_config::config::get_config;
use std::env;

mod cli;
mod constants;

#[tokio::main]
async fn main() {
    let cli = <GimCli as clap::Parser>::parse();

    // Set global verbose flag
    verbose::set_verbose(cli.verbose);

    // Only show update reminder for the main command, not for subcommands
    if env::args().nth(1).is_none_or(|arg| arg != "update")
        && let Err(e) = check_update_reminder()
    {
        eprintln!("Warning: {}", e)
    }

    // run the cli
    let config = get_config().expect("Failed to access config file");
    run_cli(&cli, config).await;
}
