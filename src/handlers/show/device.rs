// src/handlers/show/device.rs

use log::error;
use crate::app::config;
use crate::helpers::utils;
use crate::api::authentication::auth;
use crate::api::devices::{devicedetailenrichment, getdevicelist};
use crate::commands::show::device::{
    DeviceCommands, DeviceDetailFilter, DeviceEnrichmentFilter, DeviceListFilter,
};

pub fn handle_device_command(subcommand: DeviceCommands) {
    // Create a Tokio runtime
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
            DeviceCommands::List { filter } => {
                // Fetch all devices
                match getdevicelist::get_all_devices(&config, &token).await {
                    Ok(devices) => {
                        // Apply filter if necessary
                        let filtered_devices = match filter {
                            DeviceListFilter::All => devices,
                            DeviceListFilter::Hostname { partial_hostname } => {
                                devices
                                    .into_iter()
                                    .filter(|device| {
                                        if let Some(ref name) = device.hostname {
                                            if let Some(ref partial) = partial_hostname {
                                                name.contains(partial)
                                            } else {
                                                true // Include all devices with a hostname
                                            }
                                        } else {
                                            false
                                        }
                                    })
                                    .collect()
                            }
                            DeviceListFilter::Ip { partial_ip } => {
                                devices
                                    .into_iter()
                                    .filter(|device| {
                                        if let Some(ref ip) = device.management_ip_address {
                                            if let Some(ref partial) = partial_ip {
                                                ip.contains(partial)
                                            } else {
                                                true // Include all devices with an IP address
                                            }
                                        } else {
                                            false
                                        }
                                    })
                                    .collect()
                            }
                            DeviceListFilter::Wlc { partial_wlc } => {
                                devices
                                    .into_iter()
                                    .filter(|device| {
                                        if let Some(ref wlc_ip) = device.associated_wlc_ip {
                                            if let Some(ref partial) = partial_wlc {
                                                wlc_ip.contains(partial)
                                            } else {
                                                true // Include all devices with a WLC IP
                                            }
                                        } else {
                                            false
                                        }
                                    })
                                    .collect()
                            }
                        };

                        utils::print_devices(filtered_devices);
                    }
                    Err(e) => error!("Failed to retrieve devices: {}", e),
                }
            }
            DeviceCommands::Detail { filter } => {
                // Fetch all devices
                match getdevicelist::get_all_devices(&config, &token).await {
                    Ok(devices) => {
                        // Find the device matching the filter
                        let device_option = match filter {
                            DeviceDetailFilter::Hostname { ref hostname } => devices
                                .into_iter()
                                .find(|device| device.hostname.as_deref() == Some(hostname)),
                            DeviceDetailFilter::Mac { ref mac_address } => devices
                                .into_iter()
                                .find(|device| device.mac_address.as_deref() == Some(mac_address)),
                            DeviceDetailFilter::Ip { ref ip_address } => devices
                                .into_iter()
                                .find(|device| {
                                    device.management_ip_address.as_deref() == Some(ip_address)
                                }),
                        };

                        match device_option {
                            Some(device) => utils::print_device_detail(device),
                            None => println!("No device found matching the specified criteria."),
                        }
                    }
                    Err(e) => error!("Failed to retrieve devices: {}", e),
                }
            }
            DeviceCommands::Enrichment { filter } => {
                // Handle the Enrichment command
                match filter {
                    DeviceEnrichmentFilter::Mac { mac_address } => {
                        match devicedetailenrichment::get_device_enrichment(
                            &config,
                            &token,
                            "mac_address",
                            &mac_address,
                        )
                        .await
                        {
                            Ok(device_details) => {
                                utils::print_device_enrichment(device_details);
                            }
                            Err(e) => error!("Failed to retrieve device enrichment details: {}", e),
                        }
                    }
                    DeviceEnrichmentFilter::Ip { ip_address } => {
                        match devicedetailenrichment::get_device_enrichment(
                            &config,
                            &token,
                            "ip_address",
                            &ip_address,
                        )
                        .await
                        {
                            Ok(device_details) => {
                                utils::print_device_enrichment(device_details);
                            }
                            Err(e) => error!("Failed to retrieve device enrichment details: {}", e),
                        }
                    }
                }
            }
        }
    });
}
