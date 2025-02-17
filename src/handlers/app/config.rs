use log::error;
use crate::app::config;
use crate::commands::app::config::{AppConfigCommands, SetVerifySslAction};

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
                    println!("Password: [hidden]");
                    println!("Verify SSL: {}", config.verify_ssl);
                }
                Err(e) => {
                    error!("Failed to read configuration: {}", e);
                }
            }
        }
        AppConfigCommands::SetUrl { url } => {
            if let Err(e) = config::update_dnac_url(url) {
                error!("Failed to update DNA Center URL: {}", e);
            }
        }
        AppConfigCommands::SetVerifySsl { action } => {
            let enable = matches!(action, SetVerifySslAction::Enable);
            if let Err(e) = config::update_verify_ssl(enable) {
                error!("Failed to update SSL verification setting: {}", e);
            }
        }
        AppConfigCommands::ResetCredentials => {
            if let Err(e) = config::reset_credentials() {
                error!("Failed to reset credentials: {}", e);
            }
        }
    }
}
