// src/handlers/show/health.rs
use crate::api::health;
use crate::commands::show::health::HealthCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_health_command(subcommand: HealthCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            HealthCommands::Network { site_id: _ } => {
                match health::get_network_health(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(items) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row![
                                "Site Code",
                                "Health Score",
                                "Total Devices",
                                "Good",
                                "Bad",
                                "Fair"
                            ]);
                            for item in items {
                                table.add_row(row![
                                    item.site_code.as_deref().unwrap_or("N/A"),
                                    item.health_score
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    item.number_of_network_device
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    item.good_count
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    item.bad_count
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    item.fair_count
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No network health data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve network health: {}", e),
                }
            }
            HealthCommands::Client { site_id: _ } => {
                match health::get_client_health(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(items) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row![
                                "Site Code",
                                "Score Category",
                                "Score Value",
                                "Client Count"
                            ]);
                            for item in items {
                                table.add_row(row![
                                    item.site_code.as_deref().unwrap_or("N/A"),
                                    item.score_category
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    item.score_value
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                    item.client_count
                                        .as_ref()
                                        .map(|v| v.to_string())
                                        .unwrap_or_else(|| "N/A".to_string()),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No client health data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve client health: {}", e),
                }
            }
        }
        Ok(())
    });
}
