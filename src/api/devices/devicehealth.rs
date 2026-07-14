#![allow(dead_code)]
// src/api/devices/devicehealth.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceHealth {
    pub name: Option<String>,
    pub ip_address: Option<String>,
    pub device_category: Option<String>,
    pub overall_health: Option<serde_json::Value>,
    pub issue_count: Option<serde_json::Value>,
    pub device_family: Option<String>,
    pub device_type: Option<String>,
    pub os_version: Option<String>,
    pub mac_address: Option<String>,
    pub site_name: Option<String>,
    pub reachability_health: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceHealthResponse {
    pub response: Option<Vec<DeviceHealth>>,
    pub version: Option<String>,
}

pub async fn get_device_health(
    config: &Config,
    token: &Token,
    device_role: Option<&str>,
) -> Result<DeviceHealthResponse> {
    let client = http::build_client(config)?;
    let mut url = format!(
        "{}/dna/intent/api/v1/device-health?offset=1&limit=50",
        config.dnac_url
    );
    if let Some(role) = device_role {
        url.push_str(&format!("&deviceRole={}", role));
    }
    http::get_authenticated(&client, config, token, &url).await
}
