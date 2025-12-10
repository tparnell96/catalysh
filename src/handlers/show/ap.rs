// src/handlers/show/ap.rs

use crate::api::wireless::{accesspointconfig, rfprofile};
use crate::commands::show::ap::ApCommands;
use crate::helpers::{command_utils, utils};
use log::error;
use prettytable::{row, table};

pub fn handle_ap_command(subcommand: ApCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            ApCommands::Config { mac_address } => {
                // Fetch AP config
                match accesspointconfig::get_ap_config(&ctx.config, &ctx.token, &mac_address).await
                {
                    Ok(ap_config) => {
                        utils::print_ap_config(ap_config);
                    }
                    Err(e) => {
                        error!("Failed to retrieve AP config: {}", e);
                    }
                }
            }
            ApCommands::RfProfile => {
                // Fetch RF profiles
                match rfprofile::get_all_rf_profiles(&ctx.config, &ctx.token).await {
                    Ok(profiles) => {
                        println!("\nRF Profiles Overview:");
                        let mut overview_table = table!([FbFy =>
                            "Profile Name", "Default", "Channel Width", "Custom", "Brown Field",
                            "5GHz", "2.4GHz", "6GHz"
                        ]);

                        for profile in &profiles {
                            overview_table.add_row(row![
                                profile.name.as_deref().unwrap_or("N/A"),
                                if profile.default_rf_profile.unwrap_or(false) {
                                    "Yes"
                                } else {
                                    "No"
                                },
                                profile.channel_width.as_deref().unwrap_or("N/A"),
                                if profile.enable_custom.unwrap_or(false) {
                                    "Yes"
                                } else {
                                    "No"
                                },
                                if profile.enable_brown_field.unwrap_or(false) {
                                    "Yes"
                                } else {
                                    "No"
                                },
                                if profile.enable_radio_type_a.unwrap_or(false) {
                                    "✓"
                                } else {
                                    "✗"
                                },
                                if profile.enable_radio_type_b.unwrap_or(false) {
                                    "✓"
                                } else {
                                    "✗"
                                },
                                if profile.enable_radio_type_c.unwrap_or(false) {
                                    "✓"
                                } else {
                                    "✗"
                                }
                            ]);
                        }
                        overview_table.printstd();
                        overview_table.printstd();

                        for profile in &profiles {
                            println!("\nProfile: {}", profile.name.as_deref().unwrap_or("N/A"));

                            if profile.enable_radio_type_a.unwrap_or(false) {
                                println!("\n5 GHz Radio Properties:");
                                let mut radio_a_table = table!([FY =>
                                    "Parent Profile", "Channels", "Power Range", "Power Threshold",
                                    "RX SOP", "Data Rates", "Mandatory Rates"
                                ]);
                                if let Some(ref props) = profile.radio_type_a_properties {
                                    radio_a_table.add_row(row![
                                        props.parent_profile.as_deref().unwrap_or("N/A"),
                                        props.radio_channels.as_deref().unwrap_or("N/A"),
                                        format!(
                                            "{}-{}",
                                            props.min_power_level.unwrap_or(0),
                                            props.max_power_level.unwrap_or(0)
                                        ),
                                        props.power_threshold_v1.unwrap_or(0.0),
                                        props.rx_sop_threshold.as_deref().unwrap_or("N/A"),
                                        props.data_rates.as_deref().unwrap_or("N/A"),
                                        props.mandatory_data_rates.as_deref().unwrap_or("N/A")
                                    ]);
                                }
                                radio_a_table.printstd();
                            }

                            if profile.enable_radio_type_b.unwrap_or(false) {
                                println!("\n2.4 GHz Radio Properties:");
                                let mut radio_b_table = table!([FY =>
                                    "Parent Profile", "Channels", "Power Range", "Power Threshold",
                                    "RX SOP", "Data Rates", "Mandatory Rates"
                                ]);

                                if let Some(ref props) = profile.radio_type_b_properties {
                                    radio_b_table.add_row(row![
                                        props.parent_profile.as_deref().unwrap_or("N/A"),
                                        props.radio_channels.as_deref().unwrap_or("N/A"),
                                        format!(
                                            "{}-{}",
                                            props.min_power_level.unwrap_or(0),
                                            props.max_power_level.unwrap_or(0)
                                        ),
                                        props.power_threshold_v1.unwrap_or(0.0),
                                        props.rx_sop_threshold.as_deref().unwrap_or("N/A"),
                                        props.data_rates.as_deref().unwrap_or("N/A"),
                                        props.mandatory_data_rates.as_deref().unwrap_or("N/A")
                                    ]);
                                }
                                radio_b_table.printstd();
                            }

                            if profile.enable_radio_type_c.unwrap_or(false) {
                                println!("\n6 GHz Radio Properties:");
                                let mut radio_c_table = table!([FY =>
                                    "Parent Profile", "Channels", "Power Range", "Power Threshold",
                                    "RX SOP", "Data Rates", "Mandatory Rates"
                                ]);

                                if let Some(ref props) = profile.radio_type_c_properties {
                                    radio_c_table.add_row(row![
                                        props.parent_profile.as_deref().unwrap_or("N/A"),
                                        props.radio_channels.as_deref().unwrap_or("N/A"),
                                        format!(
                                            "{}-{}",
                                            props.min_power_level.unwrap_or(0),
                                            props.max_power_level.unwrap_or(0)
                                        ),
                                        props.power_threshold_v1.unwrap_or(0.0),
                                        props.rx_sop_threshold.as_deref().unwrap_or("N/A"),
                                        props.data_rates.as_deref().unwrap_or("N/A"),
                                        props.mandatory_data_rates.as_deref().unwrap_or("N/A")
                                    ]);
                                }
                                radio_c_table.printstd();
                            }
                            println!("\n");
                        }
                    }
                    Err(e) => {
                        error!("Failed to retrieve RF profiles: {}", e);
                    }
                }
            }
        }
        Ok(())
    });
}
