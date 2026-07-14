#![allow(dead_code)]
// src/api/sites/sitehealth.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteHealth {
    pub site_name: Option<String>,
    pub site_id: Option<String>,
    pub parent_site_id: Option<String>,
    pub parent_site_name: Option<String>,
    pub site_type: Option<String>,
    pub network_health_average: Option<serde_json::Value>,
    pub client_health_wired: Option<serde_json::Value>,
    pub client_health_wireless: Option<serde_json::Value>,
    pub number_of_clients: Option<serde_json::Value>,
    pub number_of_network_device: Option<serde_json::Value>,
    pub network_health_access: Option<serde_json::Value>,
    pub network_health_core: Option<serde_json::Value>,
    pub network_health_distribution: Option<serde_json::Value>,
    pub network_health_router: Option<serde_json::Value>,
    pub network_health_wireless: Option<serde_json::Value>,
    pub overall_good_devices: Option<serde_json::Value>,
    pub access: Option<serde_json::Value>,
    pub core: Option<serde_json::Value>,
    pub distribution: Option<serde_json::Value>,
    pub router: Option<serde_json::Value>,
    pub wireless_device: Option<serde_json::Value>,
    pub wired_clients: Option<serde_json::Value>,
    pub wireless_clients: Option<serde_json::Value>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteHealthResponse {
    pub response: Option<Vec<SiteHealth>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiteHealthSummary {
    pub id: Option<String>,
    pub name: Option<String>,
    pub site_hierarchy: Option<String>,
    pub site_type: Option<String>,
    pub network_good_health: Option<serde_json::Value>,
    pub network_fair_health: Option<serde_json::Value>,
    pub network_bad_health: Option<serde_json::Value>,
    pub network_total_devices: Option<serde_json::Value>,
    pub clients_good_health: Option<serde_json::Value>,
    pub clients_fair_health: Option<serde_json::Value>,
    pub clients_bad_health: Option<serde_json::Value>,
    pub clients_total_count: Option<serde_json::Value>,
    pub wired_clients: Option<serde_json::Value>,
    pub wireless_clients: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteHealthSummariesResponse {
    pub response: Option<Vec<SiteHealthSummary>>,
    pub version: Option<String>,
}

pub async fn get_site_health(
    config: &Config,
    token: &Token,
    site_type: Option<&str>,
) -> Result<SiteHealthResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/site-health", config.dnac_url);
    let mut query: Vec<(&str, String)> = Vec::new();

    if let Some(st) = site_type {
        query.push(("siteType", st.to_string()));
    }

    http::get_authenticated_with_query(&client, config, token, &url, &query).await
}

pub async fn get_site_health_by_type(
    config: &Config,
    token: &Token,
    site_type: &str,
    timestamp_ms: Option<i64>,
) -> Result<SiteHealthResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/site-health", config.dnac_url);
    let mut query = vec![("siteType", site_type.to_string())];

    if let Some(ts) = timestamp_ms {
        query.push(("timestamp", ts.to_string()));
    }

    http::get_authenticated_with_query(&client, config, token, &url, &query).await
}

pub async fn get_site_health_summaries(
    config: &Config,
    token: &Token,
    site_hierarchy: Option<&str>,
    site_type: Option<&str>,
) -> Result<SiteHealthSummariesResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/data/api/v1/siteHealthSummaries", config.dnac_url);
    let mut query: Vec<(&str, String)> = Vec::new();

    if let Some(hierarchy) = site_hierarchy {
        query.push(("siteHierarchy", hierarchy.to_string()));
    }

    if let Some(st) = site_type {
        query.push(("siteType", st.to_string()));
    }

    http::get_authenticated_with_query(&client, config, token, &url, &query).await
}
