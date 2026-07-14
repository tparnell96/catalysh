// src/handlers/show/ap.rs

use crate::api::devices::interfaces;
use crate::api::wireless::{accesspointconfig, rfprofile};
use crate::commands::show::ap::ApCommands;
use crate::helpers::{command_utils, output, resolver, utils};
use log::{error, info};
use prettytable::{row, table, Table};

pub fn handle_ap_command(subcommand: ApCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            ApCommands::Config { selector } => {
                // Resolve the selector (hostname / IP / any MAC) to the AP's ethernet MAC
                let eth_mac =
                    match resolver::resolve_ap_ethernet_mac(&ctx.config, &ctx.token, &selector)
                        .await
                    {
                        Ok(mac) => mac,
                        Err(e) => {
                            error!("Could not resolve device '{}': {}", selector, e);
                            return Ok(());
                        }
                    };

                info!("Resolved '{}' → ethernet MAC {}", selector, eth_mac);

                // Fetch AP config
                match accesspointconfig::get_ap_config(&ctx.config, &ctx.token, &eth_mac).await {
                    Ok(ap_config) => {
                        utils::print_ap_config(ap_config);
                    }
                    Err(e) => {
                        error!("Failed to retrieve AP config: {}", e);
                    }
                }
            }
            ApCommands::Neighbors { selector } => {
                // Resolve selector → device record
                let device =
                    match resolver::resolve_device(&ctx.config, &ctx.token, &selector).await {
                        Ok(d) => d,
                        Err(e) => {
                            error!("Could not resolve device '{}': {}", selector, e);
                            return Ok(());
                        }
                    };

                let device_uuid = match device.id.as_deref() {
                    Some(id) => id.to_string(),
                    None => {
                        error!("Device '{}' has no UUID in inventory", selector);
                        return Ok(());
                    }
                };

                let hostname = device.hostname.as_deref().unwrap_or(&selector);
                let mgmt_ip = device.management_ip_address.as_deref().unwrap_or("N/A");

                info!(
                    "Resolved '{}' → UUID {} ({})",
                    selector, device_uuid, hostname
                );

                match interfaces::get_device_neighbors(&ctx.config, &ctx.token, &device_uuid).await
                {
                    Ok(neighbors) if !neighbors.is_empty() => {
                        print_neighbor_table(hostname, mgmt_ip, &neighbors);
                    }
                    // Interface API returned 404 or empty (expected for APs) — fall back
                    // to the physical topology graph which always covers APs.
                    _ => {
                        match interfaces::get_neighbors_from_topology(
                            &ctx.config,
                            &ctx.token,
                            hostname,
                            mgmt_ip,
                        )
                        .await
                        {
                            Ok(neighbors) if neighbors.is_empty() => {
                                println!(
                                    "No neighbors found for {} ({}) in physical topology.",
                                    hostname, mgmt_ip
                                );
                            }
                            Ok(neighbors) => {
                                print_neighbor_table(hostname, mgmt_ip, &neighbors);
                            }
                            Err(e) => error!("Failed to retrieve topology neighbors: {}", e),
                        }
                    }
                }
            }
            ApCommands::RfProfile => {
                // Fetch RF profiles
                match rfprofile::get_all_rf_profiles(&ctx.config, &ctx.token).await {
                    Ok(profiles) => {
                        if crate::helpers::output::is_json() {
                            crate::helpers::output::print_json(&profiles);
                        } else {
                            println!("\nRF Profiles Overview:");
                            let mut overview_table = table!([FbFy =>
                                "Profile Name", "Default", "Channel Width", "Custom", "Brown Field",
                                "5GHz", "2.4GHz", "6GHz"
                            ]);

                            for profile in &profiles {
                                overview_table.add_row(row![
                                    profile.name.as_deref().unwrap_or("N/A"),
                                    if profile.default_rf_profile.unwrap_or(false) {
                                        "Yes"
                                    } else {
                                        "No"
                                    },
                                    profile.channel_width.as_deref().unwrap_or("N/A"),
                                    if profile.enable_custom.unwrap_or(false) {
                                        "Yes"
                                    } else {
                                        "No"
                                    },
                                    if profile.enable_brown_field.unwrap_or(false) {
                                        "Yes"
                                    } else {
                                        "No"
                                    },
                                    if profile.enable_radio_type_a.unwrap_or(false) {
                                        "✓"
                                    } else {
                                        "✗"
                                    },
                                    if profile.enable_radio_type_b.unwrap_or(false) {
                                        "✓"
                                    } else {
                                        "✗"
                                    },
                                    if profile.enable_radio_type_c.unwrap_or(false) {
                                        "✓"
                                    } else {
                                        "✗"
                                    }
                                ]);
                            }
                            overview_table.printstd();
                            overview_table.printstd();

                            for profile in &profiles {
                                println!("\nProfile: {}", profile.name.as_deref().unwrap_or("N/A"));

                                if profile.enable_radio_type_a.unwrap_or(false) {
                                    println!("\n5 GHz Radio Properties:");
                                    let mut radio_a_table = table!([FY =>
                                        "Parent Profile", "Channels", "Power Range", "Power Threshold",
                                        "RX SOP", "Data Rates", "Mandatory Rates"
                                    ]);
                                    if let Some(ref props) = profile.radio_type_a_properties {
                                        radio_a_table.add_row(row![
                                            props.parent_profile.as_deref().unwrap_or("N/A"),
                                            props.radio_channels.as_deref().unwrap_or("N/A"),
                                            format!(
                                                "{}-{}",
                                                props.min_power_level.unwrap_or(0),
                                                props.max_power_level.unwrap_or(0)
                                            ),
                                            props.power_threshold_v1.unwrap_or(0.0),
                                            props.rx_sop_threshold.as_deref().unwrap_or("N/A"),
                                            props.data_rates.as_deref().unwrap_or("N/A"),
                                            props.mandatory_data_rates.as_deref().unwrap_or("N/A")
                                        ]);
                                    }
                                    radio_a_table.printstd();
                                }

                                if profile.enable_radio_type_b.unwrap_or(false) {
                                    println!("\n2.4 GHz Radio Properties:");
                                    let mut radio_b_table = table!([FY =>
                                        "Parent Profile", "Channels", "Power Range", "Power Threshold",
                                        "RX SOP", "Data Rates", "Mandatory Rates"
                                    ]);

                                    if let Some(ref props) = profile.radio_type_b_properties {
                                        radio_b_table.add_row(row![
                                            props.parent_profile.as_deref().unwrap_or("N/A"),
                                            props.radio_channels.as_deref().unwrap_or("N/A"),
                                            format!(
                                                "{}-{}",
                                                props.min_power_level.unwrap_or(0),
                                                props.max_power_level.unwrap_or(0)
                                            ),
                                            props.power_threshold_v1.unwrap_or(0.0),
                                            props.rx_sop_threshold.as_deref().unwrap_or("N/A"),
                                            props.data_rates.as_deref().unwrap_or("N/A"),
                                            props.mandatory_data_rates.as_deref().unwrap_or("N/A")
                                        ]);
                                    }
                                    radio_b_table.printstd();
                                }

                                if profile.enable_radio_type_c.unwrap_or(false) {
                                    println!("\n6 GHz Radio Properties:");
                                    let mut radio_c_table = table!([FY =>
                                        "Parent Profile", "Channels", "Power Range", "Power Threshold",
                                        "RX SOP", "Data Rates", "Mandatory Rates"
                                    ]);

                                    if let Some(ref props) = profile.radio_type_c_properties {
                                        radio_c_table.add_row(row![
                                            props.parent_profile.as_deref().unwrap_or("N/A"),
                                            props.radio_channels.as_deref().unwrap_or("N/A"),
                                            format!(
                                                "{}-{}",
                                                props.min_power_level.unwrap_or(0),
                                                props.max_power_level.unwrap_or(0)
                                            ),
                                            props.power_threshold_v1.unwrap_or(0.0),
                                            props.rx_sop_threshold.as_deref().unwrap_or("N/A"),
                                            props.data_rates.as_deref().unwrap_or("N/A"),
                                            props.mandatory_data_rates.as_deref().unwrap_or("N/A")
                                        ]);
                                    }
                                    radio_c_table.printstd();
                                }
                                println!("\n");
                            }
                        } // end else (not JSON)
                    }
                    Err(e) => {
                        error!("Failed to retrieve RF profiles: {}", e);
                    }
                }
            }
        }
        Ok(())
    });
}

fn print_neighbor_table(
    device: &str,
    mgmt_ip: &str,
    neighbors: &[crate::api::devices::interfaces::DeviceNeighbor],
) {
    if output::is_json() {
        output::print_json(&neighbors.to_vec());
        return;
    }
    println!("\nNeighbors: {} ({})", device, mgmt_ip);
    let mut t = Table::new();
    t.add_row(
        row![FbFy => "Local Port", "Status", "Connected Device", "Neighbor Port", "Capabilities"],
    );
    for n in neighbors {
        t.add_row(row![
            n.local_port,
            n.port_status,
            n.neighbor_device,
            n.neighbor_port,
            n.capabilities.join(", "),
        ]);
    }
    t.printstd();
}
