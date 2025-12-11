// src/handlers/show/site.rs

use crate::api::sites::getsitelist;
use crate::commands::show::site::SiteCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, table};

pub fn handle_site_command(subcommand: SiteCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            SiteCommands::List => match getsitelist::get_all_sites(&ctx.config, &ctx.token).await {
                Ok(sites) => {
                    if sites.is_empty() {
                        println!("No sites found.");
                    } else {
                        println!("\nSites:");
                        let mut table = table!([FbFy =>
                            "Site Name", "Site Hierarchy", "Type"
                        ]);

                        for site in &sites {
                            let site_name = site.name.as_deref().unwrap_or("N/A");
                            let hierarchy = site.site_name_hierarchy.as_deref().unwrap_or("N/A");
                            let types = site
                                .group_type_list
                                .as_ref()
                                .map(|list| list.join(", "))
                                .unwrap_or_else(|| "N/A".to_string());

                            table.add_row(row![site_name, hierarchy, types]);
                        }
                        table.printstd();
                        println!("\nTotal sites: {}", sites.len());
                    }
                }
                Err(e) => error!("Failed to retrieve sites: {}", e),
            },
        }
        Ok(())
    });
}
