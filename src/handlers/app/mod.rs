pub mod config;
pub mod update;

use crate::commands::app::AppCommands;

pub fn handle_app_command(subcommand: AppCommands) {
    match subcommand {
        AppCommands::Config { subcommand } => config::handle_app_config_command(subcommand),
        AppCommands::Update => update::handle_update_command(),
    }
}
