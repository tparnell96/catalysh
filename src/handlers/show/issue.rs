// src/handlers/show/issue.rs

use crate::commands::show::issue::{IssueCommands, SearchOption};
use crate::app::config;
use crate::api::authentication::auth;
use crate::api::issues::getissuelist;
use crate::helpers::utils;
use log::error;
use std::collections::HashMap;

pub fn handle_issue_command(subcommand: IssueCommands) {
    // Create a Tokio runtime
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        // Load configuration
        let config = match config::load_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                return;
            }
        };

        // Authenticate and get token
        let token = match auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };

        match subcommand {
            IssueCommands::List { search_option, search_input } => {
                // Prepare search parameters
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
                        return;
                    }
                }

                // Fetch issue list
                match getissuelist::get_issue_list(&config, &token, &search_params).await {
                    Ok(issue_list_response) => {
                        utils::print_issue_list(issue_list_response);
                    }
                    Err(e) => {
                        error!("Failed to retrieve issue list: {}", e);
                    }
                }
            }
        }
    });
}
