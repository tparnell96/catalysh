// src/handlers/show/device.rs

use crate::api::devices::{compliance, devicecount, devicedetailenrichment, devicehealth, getdevicelist};
use crate::commands::show::device::{
    DeviceCommands, DeviceDetailFilter, DeviceEnrichmentFilter, DeviceListFilter,
};
use crate::helpers::{command_utils, utils};
use chrono::DateTime;
use log::error;
use prettytable::{row, Table};

pub fn handle_device_command(subcommand: DeviceCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            DeviceCommands::List { filter } => {
                // Fetch all devices
                match getdevicelist::get_all_devices(&ctx.config, &ctx.token).await {
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
                match getdevicelist::get_all_devices(&ctx.config, &ctx.token).await {
                    Ok(devices) => {
                        // Find the device matching the filter
                        let device_option = match filter {
                            DeviceDetailFilter::Hostname { ref hostname } => devices
                                .into_iter()
                                .find(|device| device.hostname.as_deref() == Some(hostname)),
                            DeviceDetailFilter::Mac { ref mac_address } => devices
                                .into_iter()
                                .find(|device| device.mac_address.as_deref() == Some(mac_address)),
                            DeviceDetailFilter::Ip { ref ip_address } => {
                                devices.into_iter().find(|device| {
                                    device.management_ip_address.as_deref() == Some(ip_address)
                                })
                            }
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
                            &ctx.config,
                            &ctx.token,
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
                            &ctx.config,
                            &ctx.token,
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
            DeviceCommands::Count => {
                match devicecount::get_device_count(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        let count = resp.response.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string());
                        println!("Total device count: {}", count);
                    }
                    Err(e) => error!("Failed to retrieve device count: {}", e),
                }
            }
            DeviceCommands::Health { device_role } => {
                match devicehealth::get_device_health(&ctx.config, &ctx.token, device_role.as_deref()).await {
                    Ok(resp) => {
                        if let Some(devices) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row![
                                "Name",
                                "IP Address",
                                "Category",
                                "Overall Health",
                                "Issue Count"
                            ]);
                            for d in devices {
                                table.add_row(row![
                                    d.name.as_deref().unwrap_or("N/A"),
                                    d.ip_address.as_deref().unwrap_or("N/A"),
                                    d.device_category.as_deref().unwrap_or("N/A"),
                                    d.overall_health.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                                    d.issue_count.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No device health data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve device health: {}", e),
                }
            }
            DeviceCommands::Compliance { compliance_type } => {
                match compliance::get_compliance(&ctx.config, &ctx.token, compliance_type.as_deref()).await {
                    Ok(resp) => {
                        if let Some(records) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row![
                                "Device UUID",
                                "Compliance Type",
                                "Status",
                                "Last Sync Time"
                            ]);
                            for r in records {
                                let last_sync = r.last_sync_time.map(|ts| {
                                    DateTime::from_timestamp_millis(ts)
                                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                        .unwrap_or_else(|| ts.to_string())
                                });
                                table.add_row(row![
                                    r.device_uuid.as_deref().unwrap_or("N/A"),
                                    r.compliance_type.as_deref().unwrap_or("N/A"),
                                    r.status.as_deref().unwrap_or("N/A"),
                                    last_sync.as_deref().unwrap_or("N/A"),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No compliance data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve compliance: {}", e),
                }
            }
        }
        Ok(())
    });
}
