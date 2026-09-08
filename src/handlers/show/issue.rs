// src/handlers/show/issue.rs

use crate::api::issues::getissuelist;
use crate::commands::show::issue::{IssueCommands, SearchOption};
use crate::helpers::{command_utils, resolver, utils};
use log::error;
use std::collections::HashMap;

pub fn handle_issue_command(subcommand: IssueCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            IssueCommands::List {
                search_option,
                search_input,
            } => {
                let mut search_params = HashMap::new();

                if let Some(option) = search_option {
                    if let Some(input) = search_input {
                        match option {
                            SearchOption::StartTime => {
                                search_params.insert("startTime".to_string(), input);
                            }
                            SearchOption::EndTime => {
                                search_params.insert("endTime".to_string(), input);
                            }
                            SearchOption::SiteId => {
                                search_params.insert("siteId".to_string(), input);
                            }
                            SearchOption::Device => {
                                match resolver::resolve_device_id(&ctx.config, &ctx.token, &input)
                                    .await
                                {
                                    Ok(device_id) => {
                                        search_params.insert("deviceId".to_string(), device_id);
                                    }
                                    Err(e) => {
                                        error!(
                                            "Could not resolve device selector '{}': {}",
                                            input, e
                                        );
                                        return Ok(());
                                    }
                                }
                            }
                            SearchOption::DeviceId => {
                                search_params.insert("deviceId".to_string(), input);
                            }
                            SearchOption::MacAddress => {
                                search_params.insert("macAddress".to_string(), input);
                            }
                            SearchOption::Priority => {
                                search_params.insert("priority".to_string(), input);
                            }
                            SearchOption::AiDriven => {
                                search_params.insert("aiDriven".to_string(), input);
                            }
                            SearchOption::IssueStatus => {
                                search_params.insert("issueStatus".to_string(), input);
                            }
                        }
                    } else {
                        error!("Search input is required when a search option is specified.");
                        return Ok(());
                    }
                }

                match getissuelist::get_issue_list(&ctx.config, &ctx.token, &search_params).await {
                    Ok(issue_list_response) => {
                        utils::print_issue_list(issue_list_response);
                    }
                    Err(e) => {
                        error!("Failed to retrieve issue list: {}", e);
                    }
                }
            }
        }
        Ok(())
    });
}
