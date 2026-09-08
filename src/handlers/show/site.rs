use crate::api::sites::sitehealth::{SiteHealth, SiteHealthSummary};
use crate::api::sites::{getsitelist, sitehealth};
use crate::commands::show::site::{SiteCommands, SiteHealthFilter};
use crate::helpers::command_utils;
use log::error;
use prettytable::{Table, row, table};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct SiteHealthView {
    site_id: Option<String>,
    site_name: Option<String>,
    site_hierarchy: Option<String>,
    site_type: Option<String>,
    network_health_average: Option<serde_json::Value>,
    client_health_wired: Option<serde_json::Value>,
    client_health_wireless: Option<serde_json::Value>,
    number_of_clients: Option<serde_json::Value>,
    number_of_network_device: Option<serde_json::Value>,
    network_good_health: Option<serde_json::Value>,
    network_total_devices: Option<serde_json::Value>,
}

fn json_value(value: &Option<serde_json::Value>) -> String {
    value
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_else(|| "N/A".to_string())
}

fn matches_selector(summary: &SiteHealthSummary, selector: &str) -> bool {
    let selector = selector.to_ascii_lowercase();
    summary
        .name
        .as_deref()
        .map(|name| name.eq_ignore_ascii_case(&selector))
        .unwrap_or(false)
        || summary
            .site_hierarchy
            .as_deref()
            .map(|hierarchy| hierarchy.to_ascii_lowercase().contains(&selector))
            .unwrap_or(false)
        || summary
            .id
            .as_deref()
            .map(|id| id.eq_ignore_ascii_case(&selector))
            .unwrap_or(false)
}

fn build_site_health_views(
    details: Vec<SiteHealth>,
    summaries: Vec<SiteHealthSummary>,
) -> Vec<SiteHealthView> {
    let summary_by_id: HashMap<&str, &SiteHealthSummary> = summaries
        .iter()
        .filter_map(|summary| summary.id.as_deref().map(|id| (id, summary)))
        .collect();

    details
        .into_iter()
        .map(|detail| {
            let summary = detail
                .site_id
                .as_deref()
                .and_then(|site_id| summary_by_id.get(site_id).copied());

            SiteHealthView {
                site_id: detail.site_id.clone(),
                site_name: detail
                    .site_name
                    .clone()
                    .or_else(|| summary.and_then(|item| item.name.as_ref().map(ToOwned::to_owned))),
                site_hierarchy: summary
                    .and_then(|item| item.site_hierarchy.as_ref().map(ToOwned::to_owned)),
                site_type: detail.site_type.clone().or_else(|| {
                    summary.and_then(|item| item.site_type.as_ref().map(ToOwned::to_owned))
                }),
                network_health_average: detail.network_health_average.clone(),
                client_health_wired: detail.client_health_wired.clone(),
                client_health_wireless: detail.client_health_wireless.clone(),
                number_of_clients: detail.number_of_clients.clone(),
                number_of_network_device: detail.number_of_network_device.clone(),
                network_good_health: summary.and_then(|item| item.network_good_health.clone()),
                network_total_devices: summary.and_then(|item| item.network_total_devices.clone()),
            }
        })
        .collect()
}

fn print_site_health_views(views: &[SiteHealthView]) {
    if views.is_empty() {
        println!("No site health data.");
        return;
    }

    if crate::helpers::output::is_json() {
        crate::helpers::output::print_json(views);
        return;
    }

    let mut table = Table::new();
    table.add_row(row![
        "Site Name",
        "Hierarchy",
        "Site Type",
        "Network Health Avg",
        "Wired Client Health",
        "Wireless Client Health",
        "Total Devices"
    ]);

    for site in views {
        table.add_row(row![
            site.site_name.as_deref().unwrap_or("N/A"),
            site.site_hierarchy.as_deref().unwrap_or("N/A"),
            site.site_type.as_deref().unwrap_or("N/A"),
            json_value(&site.network_health_average),
            json_value(&site.client_health_wired),
            json_value(&site.client_health_wireless),
            if site.number_of_network_device.is_some() {
                json_value(&site.number_of_network_device)
            } else {
                json_value(&site.network_total_devices)
            },
        ]);
    }

    table.printstd();
}

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
            SiteCommands::Health { filter } => match filter {
                SiteHealthFilter::All => {
                    let details =
                        sitehealth::get_site_health_by_type(&ctx.config, &ctx.token, "area", None)
                            .await;
                    let summaries = sitehealth::get_site_health_summaries(
                        &ctx.config,
                        &ctx.token,
                        None,
                        Some("area"),
                    )
                    .await;

                    match (details, summaries) {
                        (Ok(details), Ok(summaries)) => {
                            let views = build_site_health_views(
                                details.response.unwrap_or_default(),
                                summaries.response.unwrap_or_default(),
                            );
                            print_site_health_views(&views);
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            error!("Failed to retrieve site health: {}", e)
                        }
                    }
                }
                SiteHealthFilter::Building { name } => {
                    let details = sitehealth::get_site_health_by_type(
                        &ctx.config,
                        &ctx.token,
                        "building",
                        None,
                    )
                    .await;
                    let summaries = sitehealth::get_site_health_summaries(
                        &ctx.config,
                        &ctx.token,
                        None,
                        Some("building"),
                    )
                    .await;

                    match (details, summaries) {
                        (Ok(details), Ok(summaries)) => {
                            let mut views = build_site_health_views(
                                details.response.unwrap_or_default(),
                                summaries.response.unwrap_or_default(),
                            );
                            if let Some(name) = name {
                                let name = name.to_ascii_lowercase();
                                views.retain(|site| {
                                    site.site_name
                                        .as_deref()
                                        .map(|site_name| {
                                            site_name.to_ascii_lowercase().contains(&name)
                                        })
                                        .unwrap_or(false)
                                        || site
                                            .site_hierarchy
                                            .as_deref()
                                            .map(|hierarchy| {
                                                hierarchy.to_ascii_lowercase().contains(&name)
                                            })
                                            .unwrap_or(false)
                                });
                            }
                            print_site_health_views(&views);
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            error!("Failed to retrieve site health: {}", e)
                        }
                    }
                }
                SiteHealthFilter::Floor { name } => {
                    let details =
                        sitehealth::get_site_health_by_type(&ctx.config, &ctx.token, "floor", None)
                            .await;
                    let summaries = sitehealth::get_site_health_summaries(
                        &ctx.config,
                        &ctx.token,
                        None,
                        Some("floor"),
                    )
                    .await;

                    match (details, summaries) {
                        (Ok(details), Ok(summaries)) => {
                            let mut views = build_site_health_views(
                                details.response.unwrap_or_default(),
                                summaries.response.unwrap_or_default(),
                            );
                            if let Some(name) = name {
                                let name = name.to_ascii_lowercase();
                                views.retain(|site| {
                                    site.site_name
                                        .as_deref()
                                        .map(|site_name| {
                                            site_name.to_ascii_lowercase().contains(&name)
                                        })
                                        .unwrap_or(false)
                                        || site
                                            .site_hierarchy
                                            .as_deref()
                                            .map(|hierarchy| {
                                                hierarchy.to_ascii_lowercase().contains(&name)
                                            })
                                            .unwrap_or(false)
                                });
                            }
                            print_site_health_views(&views);
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            error!("Failed to retrieve site health: {}", e)
                        }
                    }
                }
                SiteHealthFilter::By { selector } => {
                    match sitehealth::get_site_health_summaries(&ctx.config, &ctx.token, None, None)
                        .await
                    {
                        Ok(summary_response) => {
                            let summaries = summary_response.response.unwrap_or_default();
                            let matching_summaries: Vec<SiteHealthSummary> = summaries
                                .into_iter()
                                .filter(|summary| matches_selector(summary, &selector))
                                .collect();

                            if matching_summaries.is_empty() {
                                println!("No site health data found for '{}'.", selector);
                            } else {
                                match sitehealth::get_site_health(&ctx.config, &ctx.token, None)
                                    .await
                                {
                                    Ok(detail_response) => {
                                        let matching_ids: HashMap<String, SiteHealthSummary> =
                                            matching_summaries
                                                .into_iter()
                                                .filter_map(|summary| {
                                                    summary.id.clone().map(|id| (id, summary))
                                                })
                                                .collect();

                                        let views = build_site_health_views(
                                            detail_response
                                                .response
                                                .unwrap_or_default()
                                                .into_iter()
                                                .filter(|detail| {
                                                    detail
                                                        .site_id
                                                        .as_ref()
                                                        .map(|site_id| {
                                                            matching_ids.contains_key(site_id)
                                                        })
                                                        .unwrap_or(false)
                                                })
                                                .collect(),
                                            matching_ids.into_values().collect(),
                                        );
                                        print_site_health_views(&views);
                                    }
                                    Err(e) => error!("Failed to retrieve site health: {}", e),
                                }
                            }
                        }
                        Err(e) => error!("Failed to retrieve site health summaries: {}", e),
                    }
                }
            },
        }
        Ok(())
    });
}
