// src/handlers/show/networksettings.rs
use crate::api::networksettings;
use crate::commands::show::networksettings::NetworkSettingsCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_networksettings_command(subcommand: NetworkSettingsCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            NetworkSettingsCommands::Global => {
                match networksettings::get_global_network_settings(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(settings) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&settings);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["Instance Type", "Key", "Value"]);
                                for s in settings {
                                    let value_str = s
                                        .value
                                        .as_ref()
                                        .map(|v| {
                                            serde_json::to_string(v)
                                                .unwrap_or_else(|_| v.to_string())
                                        })
                                        .unwrap_or_else(|| "N/A".to_string());
                                    table.add_row(row![
                                        s.instance_type.as_deref().unwrap_or("N/A"),
                                        s.key.as_deref().unwrap_or("N/A"),
                                        value_str,
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No network settings found.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve network settings: {}", e),
                }
            }
        }
        Ok(())
    });
}
