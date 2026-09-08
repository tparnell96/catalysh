// src/handlers/run.rs
use crate::api::commandrunner;
use crate::commands::run::{CommandRunnerCommands, RunCommands};
use crate::helpers::{command_utils, resolver};
use log::{error, warn};
use prettytable::{Table, row};

pub fn handle_run_command(subcommand: RunCommands) {
    match subcommand {
        RunCommands::Command { subcommand } => handle_command_runner(subcommand),
    }
}

fn handle_command_runner(subcommand: CommandRunnerCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            CommandRunnerCommands::Exec { devices, commands } => {
                // Resolve each selector to a device UUID; skip any that fail
                let mut device_uuids: Vec<String> = Vec::new();
                for selector in devices.split(',').map(|s| s.trim()) {
                    // If it already looks like a UUID (8-4-4-4-12 hex), use it directly
                    if is_uuid(selector) {
                        device_uuids.push(selector.to_string());
                    } else {
                        match resolver::resolve_device_id(&ctx.config, &ctx.token, selector).await {
                            Ok(uuid) => device_uuids.push(uuid),
                            Err(e) => warn!("Skipping '{}': {}", selector, e),
                        }
                    }
                }

                if device_uuids.is_empty() {
                    error!("No valid devices resolved — aborting.");
                    return Ok(());
                }

                match commandrunner::exec_commands(&ctx.config, &ctx.token, commands, device_uuids)
                    .await
                {
                    Ok(resp) => {
                        if let Some(inner) = resp.response {
                            let task_id = inner.task_id.as_deref().unwrap_or("unknown");
                            println!("Command runner task submitted.");
                            println!("Task ID: {}", task_id);
                            println!(
                                "Use `show task get {}` to poll status and retrieve output.",
                                task_id
                            );
                        } else {
                            println!("Submitted, but no task ID returned.");
                        }
                    }
                    Err(e) => error!("Failed to execute commands: {}", e),
                }
            }
            CommandRunnerCommands::LegitReads => {
                match commandrunner::get_legit_reads(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(commands) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row!["Allowed Read Command"]);
                            for cmd in commands {
                                table.add_row(row![cmd]);
                            }
                            table.printstd();
                        } else {
                            println!("No legit-reads returned.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve legit-reads: {}", e),
                }
            }
        }
        Ok(())
    });
}

/// Return true if the string looks like a UUID (8-4-4-4-12 hex groups).
fn is_uuid(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    matches!(parts.as_slice(), [a, b, c, d, e]
        if a.len() == 8 && b.len() == 4 && c.len() == 4 && d.len() == 4 && e.len() == 12
        && a.chars().all(|c| c.is_ascii_hexdigit())
        && b.chars().all(|c| c.is_ascii_hexdigit())
        && c.chars().all(|c| c.is_ascii_hexdigit())
        && d.chars().all(|c| c.is_ascii_hexdigit())
        && e.chars().all(|c| c.is_ascii_hexdigit()))
}
