// src/handlers/show/client.rs

use crate::commands::show::client::ClientCommands;
use crate::app::config;
use crate::api::authentication::auth;
use crate::api::clients::{getclientdetail, getclientenrichment};
use crate::helpers::utils;
use log::error;

pub fn handle_client_command(subcommand: ClientCommands) {
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
            ClientCommands::Detail { mac_address } => {
                // Fetch client details
                match getclientdetail::get_client_detail(&config, &token, &mac_address).await {
                    Ok(client_detail_response) => {
                        utils::print_client_detail(client_detail_response);
                    }
                    Err(e) => {
                        error!("Failed to retrieve client details: {}", e);
                    }
                }
            }
            ClientCommands::Enrichment {
                entity_type,
                entity_value,
                issue_category,
            } => {
                // Fetch client enrichment details
                match getclientenrichment::get_client_enrichment(
                    &config,
                    &token,
                    &entity_type,
                    &entity_value,
                    issue_category.as_deref(),
                )
                .await
                {
                    Ok(enrichment_response) => {
                        utils::print_client_enrichment(enrichment_response);
                    }
                    Err(e) => {
                        error!("Failed to retrieve client enrichment details: {}", e);
                    }
                }
            }
        }
    });
}
