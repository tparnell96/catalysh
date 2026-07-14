// src/handlers/show/platform.rs
use crate::api::platform;
use crate::commands::show::platform::PlatformCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_platform_command(subcommand: PlatformCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            PlatformCommands::Release => {
                match platform::get_release(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(rel) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&rel);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["Field", "Value"]);
                                table.add_row(row!["Name", rel.name.as_deref().unwrap_or("N/A")]);
                                table.add_row(row![
                                    "Version",
                                    rel.version.as_deref().unwrap_or("N/A")
                                ]);
                                table.add_row(row![
                                    "Installed Version",
                                    rel.installed_version.as_deref().unwrap_or("N/A")
                                ]);
                                table.add_row(row![
                                    "System Version",
                                    rel.system_version.as_deref().unwrap_or("N/A")
                                ]);
                                table.printstd();
                            }
                        } else {
                            println!("No release info available.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve platform release: {}", e),
                }
            }
            PlatformCommands::Packages => {
                match platform::get_packages(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(pkgs) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&pkgs);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["Package Name", "Version", "Install Mode"]);
                                for p in pkgs {
                                    table.add_row(row![
                                        p.name.as_deref().unwrap_or("N/A"),
                                        p.version.as_deref().unwrap_or("N/A"),
                                        p.install_mode.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No package data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve packages: {}", e),
                }
            }
        }
        Ok(())
    });
}
