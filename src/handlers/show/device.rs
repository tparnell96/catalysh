use log::error;
use crate::app::config;
use crate::helpers::utils;
use crate::api::authentication::auth;
use crate::api::devices::getdevicelist;
use crate::commands::show::device::DeviceCommands;

pub fn handle_device_command(subcommand: DeviceCommands) {
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
            DeviceCommands::All => {
                match getdevicelist::get_all_devices(&config, &token).await {
                    Ok(devices) => utils::print_devices(devices),
                    Err(e) => error!("Failed to retrieve devices: {}", e),
                }
            }
            DeviceCommands::Details { device_id } => {
                println!("Retrieving details for device: {}", device_id);
                // Add detailed device retrieval logic here
            }
        }
    });
}

