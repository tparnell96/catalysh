#![allow(dead_code)]
// src/api/advisory/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Advisory {
    pub advisory_id: Option<String>,
    pub cvss_base_score: Option<String>,
    pub publication_url: Option<String>,
    pub sir: Option<String>,
    pub cve_names: Option<Vec<String>>,
    pub bugids: Option<Vec<String>>,
    pub device_count: Option<serde_json::Value>,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdvisoryListResponse {
    pub response: Option<Vec<Advisory>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAdvisory {
    pub advisory_id: Option<String>,
    pub device_id: Option<String>,
    pub sir: Option<String>,
    pub cvss_base_score: Option<String>,
    pub publication_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceAdvisoryResponse {
    pub response: Option<Vec<DeviceAdvisory>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdvisoryAggregateResponse {
    pub response: Option<serde_json::Value>,
    pub version: Option<String>,
}

pub async fn list_advisories(config: &Config, token: &Token) -> Result<AdvisoryListResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/security-advisory/advisory",
        config.dnac_url
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_device_advisories(
    config: &Config,
    token: &Token,
    device_id: &str,
) -> Result<DeviceAdvisoryResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/security-advisory/device/{}",
        config.dnac_url, device_id
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_advisory_aggregate(
    config: &Config,
    token: &Token,
) -> Result<AdvisoryAggregateResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/security-advisory/advisory/aggregate",
        config.dnac_url
    );
    http::get_authenticated(&client, config, token, &url).await
}
