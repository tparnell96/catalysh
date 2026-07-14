// src/api/sites/sitehealth.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct SiteHealthResponse {
    pub response: Option<Vec<SiteHealth>>,
    pub version: Option<String>,
}

pub async fn get_site_health(
    config: &Config,
    token: &Token,
    site_type: Option<&str>,
) -> Result<SiteHealthResponse> {
    let client = http::build_client(config)?;
    let url = if let Some(st) = site_type {
        format!(
            "{}/dna/intent/api/v1/site-health?siteType={}",
            config.dnac_url, st
        )
    } else {
        format!("{}/dna/intent/api/v1/site-health", config.dnac_url)
    };
    http::get_authenticated(&client, config, token, &url).await
}
