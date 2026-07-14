#![allow(dead_code)]
// src/api/eox/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct EoxSummaryResponse {
    pub response: Option<HashMap<String, serde_json::Value>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EoxDevice {
    pub device_id: Option<String>,
    pub alert_count: Option<serde_json::Value>,
    pub eo_sale_date: Option<String>,
    pub eo_support_date: Option<String>,
    pub eo_sw_maintenance_releases_date: Option<String>,
    pub eo_security_vuln_support_date: Option<String>,
    pub eo_routing_failure_analysis_date: Option<String>,
    pub eo_service_contract_renewal_date: Option<String>,
    pub eo_last_hw_ship_date: Option<String>,
    pub eox_alert_summary: Option<serde_json::Value>,
    pub comments: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EoxDevicesResponse {
    pub response: Option<Vec<EoxDevice>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EoxDeviceDetailResponse {
    pub response: Option<EoxDevice>,
    pub version: Option<String>,
}

pub async fn get_eox_summary(config: &Config, token: &Token) -> Result<EoxSummaryResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/eox-status/summary", config.dnac_url);
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_eox_devices(
    config: &Config,
    token: &Token,
    limit: Option<u32>,
) -> Result<EoxDevicesResponse> {
    let client = http::build_client(config)?;
    let limit_val = limit.unwrap_or(50);
    let url = format!(
        "{}/dna/intent/api/v1/eox-status/device?limit={}",
        config.dnac_url, limit_val
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_eox_device(
    config: &Config,
    token: &Token,
    device_id: &str,
) -> Result<EoxDevicesResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/eox-status/device/{}",
        config.dnac_url, device_id
    );
    http::get_authenticated(&client, config, token, &url).await
}
