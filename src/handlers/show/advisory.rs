// src/handlers/show/advisory.rs
use crate::api::advisory;
use crate::commands::show::advisory::AdvisoryCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_advisory_command(subcommand: AdvisoryCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            AdvisoryCommands::List => {
                match advisory::list_advisories(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(advisories) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&advisories);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "Advisory ID",
                                    "CVSS Score",
                                    "SIR",
                                    "CVE Names",
                                    "Publication URL"
                                ]);
                                for a in advisories {
                                    table.add_row(row![
                                        a.advisory_id.as_deref().unwrap_or("N/A"),
                                        a.cvss_base_score.as_deref().unwrap_or("N/A"),
                                        a.sir.as_deref().unwrap_or("N/A"),
                                        a.cve_names
                                            .as_ref()
                                            .map(|v| v.join(", "))
                                            .unwrap_or_else(|| "N/A".to_string()),
                                        a.publication_url.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No advisories found.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve advisories: {}", e),
                }
            }
            AdvisoryCommands::Device { device_id } => {
                match advisory::get_device_advisories(&ctx.config, &ctx.token, &device_id).await {
                    Ok(resp) => {
                        if let Some(advisories) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&advisories);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "Advisory ID",
                                    "SIR",
                                    "CVSS Score",
                                    "Publication URL"
                                ]);
                                for a in advisories {
                                    table.add_row(row![
                                        a.advisory_id.as_deref().unwrap_or("N/A"),
                                        a.sir.as_deref().unwrap_or("N/A"),
                                        a.cvss_base_score.as_deref().unwrap_or("N/A"),
                                        a.publication_url.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No advisories for device.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve device advisories: {}", e),
                }
            }
            AdvisoryCommands::Aggregate => {
                match advisory::get_advisory_aggregate(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(agg) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&agg);
                            } else {
                                println!("Advisory Aggregate:");
                                println!(
                                    "{}",
                                    serde_json::to_string_pretty(&agg)
                                        .unwrap_or_else(|_| agg.to_string())
                                );
                            }
                        } else {
                            println!("No aggregate data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve advisory aggregate: {}", e),
                }
            }
        }
        Ok(())
    });
}
