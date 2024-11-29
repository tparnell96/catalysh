// src/handlers/show/ap.rs

use crate::commands::show::ap::ApCommands;
use crate::app::config;
use crate::api::authentication::auth;
use crate::api::wireless::accesspointconfig;
use crate::helpers::utils;
use log::error;

pub fn handle_ap_command(subcommand: ApCommands) {
    // Create a Tokio runtime
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        // Load configuration
        let config = match config::load_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                return;
            }
        };

        // Authenticate and get token
        let token = match auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };

        match subcommand {
            ApCommands::Config { mac_address } => {
                // Fetch AP config
                match accesspointconfig::get_ap_config(&config, &token, &mac_address).await {
                    Ok(ap_config) => {
                        utils::print_ap_config(ap_config);
                    }
                    Err(e) => {
                        error!("Failed to retrieve AP config: {}", e);
                    }
                }
            }
        }
    });
}
