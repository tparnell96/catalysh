// src/handlers/show/device.rs

use crate::api::devices::{
    compliance, devicecount, devicedetailenrichment, devicehealth, getdevicelist, interfaces,
};
use crate::commands::show::device::{
    DeviceCommands, DeviceDetailFilter, DeviceEnrichmentFilter, DeviceListFilter,
};
use crate::helpers::{command_utils, output, resolver, utils};
use chrono::DateTime;
use log::error;
use prettytable::{row, Table};

pub fn handle_device_command(subcommand: DeviceCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            DeviceCommands::List { filter } => {
                match getdevicelist::get_all_devices(&ctx.config, &ctx.token).await {
                    Ok(devices) => {
                        let filtered_devices = match filter {
                            DeviceListFilter::All => devices,
                            DeviceListFilter::Hostname { partial_hostname } => devices
                                .into_iter()
                                .filter(|device| {
                                    if let Some(ref name) = device.hostname {
                                        if let Some(ref partial) = partial_hostname {
                                            name.contains(partial)
                                        } else {
                                            true
                                        }
                                    } else {
                                        false
                                    }
                                })
                                .collect(),
                            DeviceListFilter::Ip { partial_ip } => devices
                                .into_iter()
                                .filter(|device| {
                                    if let Some(ref ip) = device.management_ip_address {
                                        if let Some(ref partial) = partial_ip {
                                            ip.contains(partial)
                                        } else {
                                            true
                                        }
                                    } else {
                                        false
                                    }
                                })
                                .collect(),
                            DeviceListFilter::Wlc { partial_wlc } => devices
                                .into_iter()
                                .filter(|device| {
                                    if let Some(ref wlc_ip) = device.associated_wlc_ip {
                                        if let Some(ref partial) = partial_wlc {
                                            wlc_ip.contains(partial)
                                        } else {
                                            true
                                        }
                                    } else {
                                        false
                                    }
                                })
                                .collect(),
                        };

                        utils::print_devices(filtered_devices);
                    }
                    Err(e) => error!("Failed to retrieve devices: {}", e),
                }
            }
            DeviceCommands::Detail {
                filter: DeviceDetailFilter::By { selector },
            } => match resolver::resolve_device(&ctx.config, &ctx.token, &selector).await {
                Ok(device) => utils::print_device_detail(device),
                Err(e) => error!("Could not resolve device '{}': {}", selector, e),
            },
            DeviceCommands::Enrichment {
                filter: DeviceEnrichmentFilter::By { selector },
            } => match resolver::resolve_device(&ctx.config, &ctx.token, &selector).await {
                Ok(device) => {
                    let (entity_type, entity_value) = if let Some(ref ip) = device.management_ip_address
                    {
                        ("ip_address", ip.clone())
                    } else if let Some(ref mac) = device.mac_address {
                        ("mac_address", mac.clone())
                    } else {
                        error!("Resolved device has neither IP nor MAC — cannot enrich");
                        return Ok(());
                    };
                    match devicedetailenrichment::get_device_enrichment(
                        &ctx.config,
                        &ctx.token,
                        entity_type,
                        &entity_value,
                    )
                    .await
                    {
                        Ok(details) => utils::print_device_enrichment(details),
                        Err(e) => error!("Failed to retrieve device enrichment details: {}", e),
                    }
                }
                Err(e) => error!("Could not resolve device '{}': {}", selector, e),
            },
            DeviceCommands::Neighbors { selector } => {
                let device = match resolver::resolve_device(&ctx.config, &ctx.token, &selector).await {
                    Ok(d) => d,
                    Err(e) => {
                        error!("Could not resolve device '{}': {}", selector, e);
                        return Ok(());
                    }
                };
                let device_uuid = match device.id.as_deref() {
                    Some(id) => id.to_string(),
                    None => {
                        error!("Device '{}' has no UUID", selector);
                        return Ok(());
                    }
                };
                let hostname = device.hostname.clone().unwrap_or_else(|| selector.clone());
                let mgmt_ip = device
                    .management_ip_address
                    .clone()
                    .unwrap_or_else(|| "N/A".to_string());

                let neighbors = match interfaces::get_device_neighbors(
                    &ctx.config,
                    &ctx.token,
                    &device_uuid,
                )
                .await
                {
                    Ok(n) if !n.is_empty() => n,
                    _ => match interfaces::get_neighbors_from_topology(
                        &ctx.config,
                        &ctx.token,
                        &hostname,
                        &mgmt_ip,
                    )
                    .await
                    {
                        Ok(n) => n,
                        Err(e) => {
                            error!("Failed to retrieve neighbors: {}", e);
                            return Ok(());
                        }
                    },
                };

                if neighbors.is_empty() {
                    println!("No neighbors found for {} ({})", hostname, mgmt_ip);
                } else if output::is_json() {
                    output::print_json(&neighbors);
                } else {
                    println!("
Neighbors: {} ({})", hostname, mgmt_ip);
                    let mut t = Table::new();
                    t.add_row(row![FbFy => "Local Port", "Status", "Connected Device", "Neighbor Port", "Capabilities"]);
                    for n in &neighbors {
                        t.add_row(row![
                            n.local_port,
                            n.port_status,
                            n.neighbor_device,
                            n.neighbor_port,
                            n.capabilities.join(", ")
                        ]);
                    }
                    t.printstd();
                }
            }
            DeviceCommands::Count => {
                match devicecount::get_device_count(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        let count = resp
                            .response
                            .as_ref()
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "N/A".to_string());
                        println!("Total device count: {}", count);
                    }
                    Err(e) => error!("Failed to retrieve device count: {}", e),
                }
            }
            DeviceCommands::Health { device_role } => {
                match devicehealth::get_device_health(
                    &ctx.config,
                    &ctx.token,
                    device_role.as_deref(),
                )
                .await
                {
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
                                    d.overall_health
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    d.issue_count
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
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
                match compliance::get_compliance(
                    &ctx.config,
                    &ctx.token,
                    compliance_type.as_deref(),
                )
                .await
                {
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
