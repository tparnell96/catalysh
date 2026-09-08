use crate::api::health;
use crate::commands::show::health::HealthCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{Table, row};

fn json_value(value: &Option<serde_json::Value>) -> String {
    value
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

pub fn handle_health_command(subcommand: HealthCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            HealthCommands::Network { site_id: _ } => {
                match health::get_network_health(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(items) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&items);
                            } else {
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
                                        json_value(&item.health_score),
                                        json_value(&item.number_of_network_device),
                                        json_value(&item.good_count),
                                        json_value(&item.bad_count),
                                        json_value(&item.fair_count),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No network health data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve network health: {}", e),
                }
            }
            HealthCommands::Client { timestamp } => {
                match health::get_client_health(&ctx.config, &ctx.token, timestamp).await {
                    Ok(resp) => {
                        if resp.response.is_empty() {
                            println!("No client health data.");
                        } else if crate::helpers::output::is_json() {
                            crate::helpers::output::print_json(&resp);
                        } else {
                            let mut table = Table::new();
                            table.add_row(row![
                                "Site ID",
                                "Category",
                                "Category Value",
                                "Score",
                                "Client Count",
                                "Unique Clients",
                                "Start",
                                "End"
                            ]);

                            for site in resp.response {
                                if site.score_detail.is_empty() {
                                    table.add_row(row![
                                        site.site_id.as_deref().unwrap_or("N/A"),
                                        "N/A",
                                        "N/A",
                                        "N/A",
                                        "N/A",
                                        "N/A",
                                        "N/A",
                                        "N/A"
                                    ]);
                                    continue;
                                }

                                for score in site.score_detail {
                                    table.add_row(row![
                                        site.site_id.as_deref().unwrap_or("N/A"),
                                        score
                                            .score_category
                                            .as_ref()
                                            .and_then(|category| category.score_category.as_deref())
                                            .unwrap_or("N/A"),
                                        score
                                            .score_category
                                            .as_ref()
                                            .and_then(|category| category.value.as_ref())
                                            .map(|value| value.to_string())
                                            .unwrap_or_else(|| "N/A".to_string()),
                                        json_value(&score.score_value),
                                        json_value(&score.client_count),
                                        json_value(&score.client_unique_count),
                                        score
                                            .starttime
                                            .map(|value| value.to_string())
                                            .unwrap_or_else(|| "N/A".to_string()),
                                        score
                                            .endtime
                                            .map(|value| value.to_string())
                                            .unwrap_or_else(|| "N/A".to_string()),
                                    ]);
                                }
                            }

                            table.printstd();
                        }
                    }
                    Err(e) => error!("Failed to retrieve client health: {}", e),
                }
            }
        }
        Ok(())
    });
}
