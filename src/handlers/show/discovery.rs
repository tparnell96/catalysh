// src/handlers/show/discovery.rs
use crate::api::discovery;
use crate::commands::show::discovery::DiscoveryCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_discovery_command(subcommand: DiscoveryCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            DiscoveryCommands::List { limit } => {
                match discovery::list_discoveries(&ctx.config, &ctx.token, limit).await {
                    Ok(resp) => {
                        if let Some(discoveries) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row![
                                "ID",
                                "Name",
                                "Type",
                                "IP Address List",
                                "Status"
                            ]);
                            for d in discoveries {
                                table.add_row(row![
                                    d.id.as_deref().unwrap_or("N/A"),
                                    d.name.as_deref().unwrap_or("N/A"),
                                    d.discovery_type.as_deref().unwrap_or("N/A"),
                                    d.ip_address_list.as_deref().unwrap_or("N/A"),
                                    d.discovery_status.as_deref().unwrap_or("N/A"),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No discoveries found.");
                        }
                    }
                    Err(e) => error!("Failed to list discoveries: {}", e),
                }
            }
            DiscoveryCommands::Get { id } => {
                match discovery::get_discovery(&ctx.config, &ctx.token, &id).await {
                    Ok(resp) => {
                        if let Some(d) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row!["Field", "Value"]);
                            table.add_row(row!["ID", d.id.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["Name", d.name.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["Type", d.discovery_type.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["IP List", d.ip_address_list.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["Status", d.discovery_status.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["Device IDs", d.device_ids.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["Protocol Order", d.protocol_order.as_deref().unwrap_or("N/A")]);
                            table.add_row(row!["Netconf Port", d.netconf_port.as_deref().unwrap_or("N/A")]);
                            table.printstd();
                        } else {
                            println!("Discovery not found.");
                        }
                    }
                    Err(e) => error!("Failed to get discovery: {}", e),
                }
            }
        }
        Ok(())
    });
}
