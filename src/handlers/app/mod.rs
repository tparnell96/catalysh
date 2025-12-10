pub mod config;
pub mod update;

use crate::commands::app::AppCommands;

pub fn handle_app_command(subcommand: AppCommands) {
    match subcommand {
        AppCommands::Config { subcommand } => config::handle_app_config_command(subcommand),
        AppCommands::Update => update::handle_update_command(),
        AppCommands::Version => handle_version_command(),
    }
}

fn handle_version_command() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");

    println!("{} version {}", NAME, VERSION);
    println!("A command line utility for interacting with Cisco Catalyst Center");
}
