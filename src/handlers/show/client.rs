// src/handlers/show/client.rs

use crate::api::clients::{getclientdetail, getclientenrichment};
use crate::commands::show::client::ClientCommands;
use crate::helpers::{command_utils, utils};
use log::error;

pub fn handle_client_command(subcommand: ClientCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            ClientCommands::Detail { mac_address } => {
                // Fetch client details
                match getclientdetail::get_client_detail(&ctx.config, &ctx.token, &mac_address)
                    .await
                {
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
                    &ctx.config,
                    &ctx.token,
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
        Ok(())
    });
}
