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
        AppConfigCommands::Show => {
            match config::load_config() {
                Ok(config) => {
                    println!("Current Configuration:");
                    println!("---------------------");
                    println!("DNA Center URL: {}", config.dnac_url);
                    println!("Username: {}", config.username);
                    // Don't show the password for security
                    println!("Password: [hidden]");
                    println!("Verify SSL: {}", config.verify_ssl);
                }
                Err(e) => {
                    error!("Failed to read configuration: {}", e);
                }
            }
        }
    }
}

