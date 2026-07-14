// src/handlers/show/eox.rs
use crate::api::eox;
use crate::commands::show::eox::EoxCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_eox_command(subcommand: EoxCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            EoxCommands::Summary => {
                match eox::get_eox_summary(&ctx.config, &ctx.token).await {
                    Ok(resp) => {
                        if let Some(summary) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&summary);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["Key", "Value"]);
                                for (k, v) in &summary {
                                    table.add_row(row![k, v.to_string()]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No EoX summary data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve EoX summary: {}", e),
                }
            }
            EoxCommands::Devices { limit } => {
                match eox::get_eox_devices(&ctx.config, &ctx.token, limit).await {
                    Ok(resp) => {
                        if let Some(devices) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&devices);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "Device ID",
                                    "Alert Count",
                                    "EoSale Date",
                                    "EoSupport Date"
                                ]);
                                for d in devices {
                                    table.add_row(row![
                                        d.device_id.as_deref().unwrap_or("N/A"),
                                        d.alert_count
                                            .as_ref()
                                            .map(|v| v.to_string())
                                            .unwrap_or_else(|| "N/A".to_string()),
                                        d.eo_sale_date.as_deref().unwrap_or("N/A"),
                                        d.eo_support_date.as_deref().unwrap_or("N/A"),
                                    ]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No EoX device data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve EoX devices: {}", e),
                }
            }
            EoxCommands::Device { device_id } => {
                match eox::get_eox_device(&ctx.config, &ctx.token, &device_id).await {
                    Ok(resp) => {
                        if let Some(devices) = resp.response {
                            if crate::helpers::output::is_json() {
                                crate::helpers::output::print_json(&devices);
                            } else {
                                let mut table = Table::new();
                                table.add_row(row!["Field", "Value"]);
                                for d in devices {
                                    table.add_row(row!["Device ID", d.device_id.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["Alert Count", d.alert_count.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string())]);
                                    table.add_row(row!["EoSale Date", d.eo_sale_date.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["EoSupport Date", d.eo_support_date.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["EoSW Maintenance Date", d.eo_sw_maintenance_releases_date.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["EoSecurity Vuln Support", d.eo_security_vuln_support_date.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["EoService Contract Renewal", d.eo_service_contract_renewal_date.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["EoLast HW Ship Date", d.eo_last_hw_ship_date.as_deref().unwrap_or("N/A")]);
                                    table.add_row(row!["Comments", d.comments.as_deref().unwrap_or("N/A")]);
                                }
                                table.printstd();
                            }
                        } else {
                            println!("No EoX data for device.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve EoX device detail: {}", e),
                }
            }
        }
        Ok(())
    });
}
