// src/handlers/show/path.rs
use crate::api::pathanalysis;
use crate::commands::show::path::PathCommands;
use crate::helpers::command_utils;
use chrono::DateTime;
use log::error;
use prettytable::{row, Table};

pub fn handle_path_command(subcommand: PathCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            PathCommands::List => {
                match pathanalysis::list_flow_analyses(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(flows) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&flows);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "ID", "Source IP", "Dest IP", "Status", "Created"
                                ]);
                                for f in flows {
                                    let created = f.create_time.map(|ts| {
                                        DateTime::from_timestamp_millis(ts)
                                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                            .unwrap_or_else(|| ts.to_string())
                                    });
                                    table.add_row(row![
                                        f.id.as_deref().unwrap_or("N/A"),
                                        f.source_i_p.as_deref().unwrap_or("N/A"),
                                        f.dest_i_p.as_deref().unwrap_or("N/A"),
                                        f.status.as_deref().unwrap_or("N/A"),
                                        created.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No flow analyses found.");
                        }
                    }
                    Err(e) => error!("Failed to list flow analyses: {}", e),
                }
            }
            PathCommands::Trace {
                source_ip,
                dest_ip,
                source_port,
                dest_port,
                protocol,
            } => {
                match pathanalysis::start_path_trace(
                    &ctx.config,
                    &ctx.token,
                    &source_ip,
                    &dest_ip,
                    source_port.as_deref(),
                    dest_port.as_deref(),
                    protocol.as_deref(),
                )
                .await
                {
                    Ok(resp) => {
                        if let Some(inner) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&inner);
                            } else {
                                let fid = inner.flow_analysis_id.as_deref().unwrap_or("unknown");
                                println!("Path trace started. Flow Analysis ID: {}", fid);
                                println!("Use `show path get {}` to retrieve results.", fid);
                            }
                        } else {
                            println!("Path trace submitted but no flow analysis ID returned.");
                        }
                    }
                    Err(e) => error!("Failed to start path trace: {}", e),
                }
            }
            PathCommands::Get { flow_id } => {
                match pathanalysis::get_flow_analysis(&ctx.config, &ctx.token, &flow_id).await {
                    Ok(resp) => {
                        if let Some(detail) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&detail);
                            } else {
                                println!("Flow Analysis: {}", detail.id.as_deref().unwrap_or("N/A"));
                                println!("  Source: {}", detail.source_i_p.as_deref().unwrap_or("N/A"));
                                println!("  Dest:   {}", detail.dest_i_p.as_deref().unwrap_or("N/A"));
                                println!("  Status: {}", detail.status.as_deref().unwrap_or("N/A"));
                                if let Some(elements) = detail.network_elements_info {
                                    println!("\nPath Hops:");
                                    let mut table = Table::new();
                                    table.add_row(row!["Name", "IP", "Type", "Role"]);
                                    for elem in elements {
                                        table.add_row(row![
                                            elem.name.as_deref().unwrap_or("N/A"),
                                            elem.ip.as_deref().unwrap_or("N/A"),
                                            elem.r#type.as_deref().unwrap_or("N/A"),
                                            elem.role.as_deref().unwrap_or("N/A"),
                                        ]);
                                    }
                                    table.printstd();
                                }
                            }
                        } else {
                            println!("Flow analysis not found.");
                        }
                    }
                    Err(e) => error!("Failed to get flow analysis: {}", e),
                }
            }
        }
        Ok(())
    });
}
