// src/handlers/run.rs
use crate::api::commandrunner;
use crate::commands::run::{CommandRunnerCommands, RunCommands};
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_run_command(subcommand: RunCommands) {
    match subcommand {
        RunCommands::Command { subcommand } => handle_command_runner(subcommand),
    }
}

fn handle_command_runner(subcommand: CommandRunnerCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            CommandRunnerCommands::Exec { devices, commands } => {
                let device_uuids: Vec<String> =
                    devices.split(',').map(|s| s.trim().to_string()).collect();
                match commandrunner::exec_commands(
                    &ctx.config,
                    &ctx.token,
                    commands,
                    device_uuids,
                )
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
