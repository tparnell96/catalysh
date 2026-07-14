// src/handlers/show/tag.rs
use crate::api::tags;
use crate::commands::show::tag::TagCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, Table};

pub fn handle_tag_command(subcommand: TagCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            TagCommands::List { name } => {
                match tags::list_tags(&ctx.config, &ctx.token, name.as_deref()).await {
                    Ok(resp) => {
                        if let Some(tag_list) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row!["ID", "Name", "Description"]);
                            for t in tag_list {
                                table.add_row(row![
                                    t.id.as_deref().unwrap_or("N/A"),
                                    t.name.as_deref().unwrap_or("N/A"),
                                    t.description.as_deref().unwrap_or("N/A"),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No tags found.");
                        }
                    }
                    Err(e) => error!("Failed to list tags: {}", e),
                }
            }
            TagCommands::Members { tag_id } => {
                match tags::get_tag_members(&ctx.config, &ctx.token, &tag_id).await {
                    Ok(resp) => {
                        if let Some(members) = resp.response {
                            println!("Tag members for {}:", tag_id);
                            println!(
                                "{}",
                                serde_json::to_string_pretty(&members)
                                    .unwrap_or_else(|_| members.to_string())
                            );
                        } else {
                            println!("No members found for tag.");
                        }
                    }
                    Err(e) => error!("Failed to get tag members: {}", e),
                }
            }
        }
        Ok(())
    });
}
