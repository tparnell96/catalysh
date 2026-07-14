// src/handlers/show/client.rs

use crate::api::clients::{clientlist, clientproximity, getclientdetail, getclientenrichment};
use crate::commands::show::client::ClientCommands;
use crate::helpers::{command_utils, resolver, utils};
use log::error;

pub fn handle_client_command(subcommand: ClientCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            ClientCommands::Detail { selector } => {
                let mac_address = match resolver::resolve_device(&ctx.config, &ctx.token, &selector).await {
                    Ok(device) => device.mac_address.unwrap_or(selector.clone()),
                    Err(_) => selector.clone(),
                };

                match getclientdetail::get_client_detail(
                    &ctx.config,
                    &ctx.token,
                    &mac_address,
                    None,
                )
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
            ClientCommands::List {
                mac,
                ipv4,
                ssid,
                client_type,
                site_id,
                limit,
            } => {
                match clientlist::get_client_list(
                    &ctx.config,
                    &ctx.token,
                    mac.as_deref(),
                    ipv4.as_deref(),
                    None,
                    ssid.as_deref(),
                    client_type.as_deref(),
                    site_id.as_deref(),
                    limit,
                    0,
                )
                .await
                {
                    Ok(clients) => {
                        utils::print_client_list(clients);
                    }
                    Err(e) => {
                        error!("Failed to retrieve client list: {}", e);
                    }
                }
            }
            ClientCommands::Proximity { username, days } => {
                match clientproximity::get_client_proximity(
                    &ctx.config,
                    &ctx.token,
                    &username,
                    Some(days),
                    None,
                )
                .await
                {
                    Ok(resp) => {
                        if crate::helpers::output::is_json() {
                            crate::helpers::output::print_json(&resp);
                        } else {
                            println!(
                                "Execution ID: {}",
                                resp.execution_id.as_deref().unwrap_or("N/A")
                            );
                            println!(
                                "Status URL:   {}",
                                resp.execution_status_url.as_deref().unwrap_or("N/A")
                            );
                            if let Some(msg) = &resp.message {
                                println!("Message:      {}", msg);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to retrieve client proximity: {}", e);
                    }
                }
            }
        }
        Ok(())
    });
}
