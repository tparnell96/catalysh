// src/handlers/show/task.rs
use crate::api::task;
use crate::commands::show::task::TaskCommands;
use crate::helpers::command_utils;
use chrono::DateTime;
use log::error;
use prettytable::{row, Table};

pub fn handle_task_command(subcommand: TaskCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            TaskCommands::List { offset, limit } => {
                match task::list_tasks(&ctx.config, &ctx.token, offset, limit).await {
                    Ok(resp) => {
                        if let Some(tasks) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&tasks);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "ID",
                                    "Service Type",
                                    "Is Error",
                                    "Start Time",
                                    "End Time",
                                    "Progress"
                                ]);
                                for t in tasks {
                                    let start = t.start_time.map(|ts| {
                                        DateTime::from_timestamp_millis(ts)
                                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                            .unwrap_or_else(|| ts.to_string())
                                    });
                                    let end = t.end_time.map(|ts| {
                                        DateTime::from_timestamp_millis(ts)
                                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                            .unwrap_or_else(|| ts.to_string())
                                    });
                                    table.add_row(row![
                                        t.id.as_deref().unwrap_or("N/A"),
                                        t.service_type.as_deref().unwrap_or("N/A"),
                                        t.is_error
                                            .map(|b| b.to_string())
                                            .unwrap_or_else(|| "N/A".to_string()),
                                        start.as_deref().unwrap_or("N/A"),
                                        end.as_deref().unwrap_or("N/A"),
                                        t.progress.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No tasks found.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve task list: {}", e),
                }
            }
            TaskCommands::Get { task_id } => {
                match task::get_task(&ctx.config, &ctx.token, &task_id).await {
                    Ok(resp) => {
                        if let Some(t) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&t);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["Field", "Value"]);
                                table.add_row(row!["ID", t.id.as_deref().unwrap_or("N/A")]);
                                table.add_row(row![
                                    "Service Type",
                                    t.service_type.as_deref().unwrap_or("N/A")
                                ]);
                                table.add_row(row![
                                    "Is Error",
                                    t.is_error
                                        .map(|b| b.to_string())
                                        .unwrap_or_else(|| "N/A".to_string())
                                ]);
                                table.add_row(row![
                                    "Progress",
                                    t.progress.as_deref().unwrap_or("N/A")
                                ]);
                                table.add_row(row!["Data", t.data.as_deref().unwrap_or("N/A")]);
                                table.add_row(row![
                                    "Error Code",
                                    t.error_code.as_deref().unwrap_or("N/A")
                                ]);
                                table.add_row(row![
                                    "Failure Reason",
                                    t.failure_reason.as_deref().unwrap_or("N/A")
                                ]);
                                table.add_row(row![
                                    "Additional Status URL",
                                    t.additional_status_url.as_deref().unwrap_or("N/A")
                                ]);
                                if let Some(ts) = t.start_time {
                                    let dt = DateTime::from_timestamp_millis(ts)
                                        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                                        .unwrap_or_else(|| ts.to_string());
                                    table.add_row(row!["Start Time", dt]);
                                }
                                if let Some(ts) = t.end_time {
                                    let dt = DateTime::from_timestamp_millis(ts)
                                        .map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
                                        .unwrap_or_else(|| ts.to_string());
                                    table.add_row(row!["End Time", dt]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("Task not found.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve task: {}", e),
                }
            }
        }
        Ok(())
    });
}
