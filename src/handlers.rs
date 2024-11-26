use log::error;
use crate::config;
use crate::utils;
use crate::update;
use crate::api::authentication::auth;
use crate::api::devices::getdevicelist;

use super::commands::{DeviceSubcommands, ConfigSubcommands};

pub fn handle_devices(subcommand: DeviceSubcommands) {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        let config = match config::load_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                return;
            }
        };

        let token = match auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };

        match subcommand {
            DeviceSubcommands::All => {
                match getdevicelist::get_all_devices(&config, &token).await {
                    Ok(devices) => utils::print_devices(devices),
                    Err(e) => error!("Failed to retrieve devices: {}", e),
                }
            }
            DeviceSubcommands::Details { device_id } => {
                println!("Retrieving details for device: {}", device_id);
                // Add detailed device retrieval logic here
            }
        }
    });
}

pub fn handle_config(subcommand: ConfigSubcommands) {
    match subcommand {
        ConfigSubcommands::Reset => {
            if let Err(e) = config::reset_config() {
                error!("Failed to reset configuration: {}", e);
            } else {
                println!("Configuration reset successfully.");
            }
        }
    }
}

pub fn handle_update() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        if let Err(e) = update::update_to_latest() {
            eprintln!("Update failed: {}", e);
        } else {
            println!("Update completed successfully.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("Please download and run the latest `windows_installer.exe` to update the application.");
    }
}
