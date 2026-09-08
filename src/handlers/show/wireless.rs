// src/handlers/show/wireless.rs
use crate::api::wireless::ssids;
use crate::commands::show::wireless::WirelessCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{Table, row};

pub fn handle_wireless_command(subcommand: WirelessCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            WirelessCommands::Ssid { site_id } => {
                match ssids::get_ssids(&ctx.config, &ctx.token, &site_id).await {
                    Ok(resp) => {
                        if let Some(ssid_list) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&ssid_list);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["SSID", "Security Level", "Passphrase"]);
                                for s in ssid_list {
                                    let masked = s.passphrase.as_ref().map(|p| {
                                        if p.is_empty() {
                                            "N/A".to_string()
                                        } else {
                                            "*".repeat(8)
                                        }
                                    });
                                    table.add_row(row![
                                        s.ssid
                                            .as_deref()
                                            .unwrap_or(s.name.as_deref().unwrap_or("N/A")),
                                        s.security_level.as_deref().unwrap_or("N/A"),
                                        masked.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No SSIDs found for site.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve SSIDs: {}", e),
                }
            }
        }
        Ok(())
    });
}
