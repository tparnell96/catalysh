#![allow(dead_code)]
// src/api/discovery/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discovery {
    pub id: Option<String>,
    pub name: Option<String>,
    pub discovery_type: Option<String>,
    pub ip_address_list: Option<String>,
    pub discovery_status: Option<String>,
    pub device_ids: Option<String>,
    pub snmp_ro_community: Option<String>,
    pub protocol_order: Option<String>,
    pub retry: Option<serde_json::Value>,
    pub timeout: Option<serde_json::Value>,
    pub num_devices: Option<serde_json::Value>,
    pub credential_id_list: Option<serde_json::Value>,
    pub global_credential_id_list: Option<Vec<String>>,
    pub is_auto_cdp: Option<bool>,
    pub cdp_level: Option<serde_json::Value>,
    pub lldp_level: Option<serde_json::Value>,
    pub parent_discovery_id: Option<String>,
    pub user_name_list: Option<String>,
    pub password_list: Option<String>,
    pub ip_filter_list: Option<String>,
    pub http_read_credential: Option<serde_json::Value>,
    pub http_write_credential: Option<serde_json::Value>,
    pub start_index: Option<serde_json::Value>,
    pub records_to_return: Option<serde_json::Value>,
    pub condition_for_network_scan: Option<String>,
    pub netconf_port: Option<String>,
    pub prefer_management_ip_address: Option<bool>,
    pub enable_password_list: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryListResponse {
    pub response: Option<Vec<Discovery>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiscoveryDetailResponse {
    pub response: Option<Discovery>,
    pub version: Option<String>,
}

pub async fn list_discoveries(
    config: &Config,
    token: &Token,
    limit: Option<u32>,
) -> Result<DiscoveryListResponse> {
    let client = http::build_client(config)?;
    let records = limit.unwrap_or(10);
    let url = format!(
        "{}/dna/intent/api/v1/discovery?Records={}&StartIndex=1",
        config.dnac_url, records
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_discovery(
    config: &Config,
    token: &Token,
    id: &str,
) -> Result<DiscoveryDetailResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/discovery/{}", config.dnac_url, id);
    http::get_authenticated(&client, config, token, &url).await
}
