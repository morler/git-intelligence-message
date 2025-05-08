use cli::{command::Cli, entry::run_cli};
use config::get_config_into_toml;

mod cli;
mod config;

fn main() {
    let config = get_config_into_toml().expect("Failed to access config file");
    let cli = <Cli as clap::Parser>::parse();
    run_cli(&cli, config);
}
