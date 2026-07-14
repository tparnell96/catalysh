// src/handlers/show/topology.rs
use crate::api::topology;
use crate::commands::show::topology::TopologyCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_topology_command(subcommand: TopologyCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            TopologyCommands::Physical => {
                match topology::get_physical_topology(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(graph) = resp.response {
                            if let Some(nodes) = graph.nodes {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "Label", "IP", "Device Type", "Role", "Family"
                                ]);
                                for node in nodes {
                                    table.add_row(row![
                                        node.label.as_deref().unwrap_or("N/A"),
                                        node.ip.as_deref().unwrap_or("N/A"),
                                        node.device_type.as_deref().unwrap_or("N/A"),
                                        node.role.as_deref().unwrap_or("N/A"),
                                        node.family.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            } else {
                                println!("No nodes in physical topology.");
                            }
                        } else {
                            println!("No physical topology data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve physical topology: {}", e),
                }
            }
            TopologyCommands::Sites => {
                match topology::get_site_topology(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(inner) = resp.response {
                            if let Some(sites) = inner.sites {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "Name", "ID", "Location Address", "Parent ID"
                                ]);
                                for site in sites {
                                    table.add_row(row![
                                        site.name.as_deref().unwrap_or("N/A"),
                                        site.id.as_deref().unwrap_or("N/A"),
                                        site.location_address.as_deref().unwrap_or("N/A"),
                                        site.parent_id.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            } else {
                                println!("No sites in topology.");
                            }
                        } else {
                            println!("No site topology data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve site topology: {}", e),
                }
            }
            TopologyCommands::L3 { topology_type } => {
                match topology::get_l3_topology(&ctx.config, &ctx.token, &topology_type).await {
                    Ok(resp) => {
                        if let Some(graph) = resp.response {
                            if let Some(nodes) = graph.nodes {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "Label", "IP", "Device Type", "Role", "Family"
                                ]);
                                for node in nodes {
                                    table.add_row(row![
                                        node.label.as_deref().unwrap_or("N/A"),
                                        node.ip.as_deref().unwrap_or("N/A"),
                                        node.device_type.as_deref().unwrap_or("N/A"),
                                        node.role.as_deref().unwrap_or("N/A"),
                                        node.family.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            } else {
                                println!("No nodes in L3 topology.");
                            }
                        } else {
                            println!("No L3 topology data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve L3 topology: {}", e),
                }
            }
            TopologyCommands::Vlans => {
                match topology::get_vlan_names(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(vlans) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row!["VLAN Name"]);
                            for vlan in vlans {
                                table.add_row(row![vlan]);
                            }
                            table.printstd();
                        } else {
                            println!("No VLAN names returned.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve VLAN names: {}", e),
                }
            }
        }
        Ok(())
    });
}
