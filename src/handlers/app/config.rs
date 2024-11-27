use log::error;
use crate::app::config; // Adjusted import
use crate::commands::app::config::AppConfigCommands;

pub fn handle_app_config_command(subcommand: AppConfigCommands) {
    match subcommand {
        AppConfigCommands::Reset => {
            if let Err(e) = config::reset_config() {
                error!("Failed to reset configuration: {}", e);
            } else {
                println!("Configuration reset successfully.");
            }
        }
    }
}

