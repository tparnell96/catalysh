// src/handlers/show/site.rs

use crate::api::sites::{getsitelist, sitehealth};
use crate::commands::show::site::SiteCommands;
use crate::helpers::command_utils;
use log::error;
use prettytable::{row, table, Table};

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
            SiteCommands::Health { site_type } => {
                match sitehealth::get_site_health(&ctx.config, &ctx.token, site_type.as_deref()).await {
                    Ok(resp) => {
                        if let Some(sites) = resp.response {
                            let mut table = Table::new();
                            table.add_row(row![
                                "Site Name",
                                "Site Type",
                                "Network Health Avg",
                                "Wired Client Health",
                                "Wireless Client Health"
                            ]);
                            for s in sites {
                                table.add_row(row![
                                    s.site_name.as_deref().unwrap_or("N/A"),
                                    s.site_type.as_deref().unwrap_or("N/A"),
                                    s.network_health_average.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                                    s.client_health_wired.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                                    s.client_health_wireless.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string()),
                                ]);
                            }
                            table.printstd();
                        } else {
                            println!("No site health data.");
                        }
                    }
                    Err(e) => error!("Failed to retrieve site health: {}", e),
                }
            }
        }
        Ok(())
    });
}
