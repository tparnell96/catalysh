#![allow(dead_code)]
// src/api/health/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::{http, utils};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkHealthItem {
    pub health_score: Option<serde_json::Value>,
    pub site_code: Option<String>,
    pub number_of_network_device: Option<serde_json::Value>,
    pub good_count: Option<serde_json::Value>,
    pub bad_count: Option<serde_json::Value>,
    pub fair_count: Option<serde_json::Value>,
    pub unmon_count: Option<serde_json::Value>,
    pub total_count: Option<serde_json::Value>,
    pub no_health_count: Option<serde_json::Value>,
    pub goodpercentage: Option<serde_json::Value>,
    pub badpercentage: Option<serde_json::Value>,
    pub fairpercentage: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkHealthResponse {
    pub response: Option<Vec<NetworkHealthItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientHealthItem {
    pub site_code: Option<String>,
    pub score_category: Option<serde_json::Value>,
    pub score_value: Option<serde_json::Value>,
    pub client_count: Option<serde_json::Value>,
    pub client_unique_count: Option<serde_json::Value>,
    pub starttime: Option<i64>,
    pub endtime: Option<i64>,
    pub connected_to_udncount: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientHealthResponse {
    pub response: Option<Vec<ClientHealthItem>>,
}

pub async fn get_network_health(config: &Config, token: &Token) -> Result<NetworkHealthResponse> {
    let client = http::build_client(config)?;
    let ts = utils::current_timestamp();
    let url = format!(
        "{}/dna/intent/api/v1/network-health?timestamp={}",
        config.dnac_url, ts
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_client_health(config: &Config, token: &Token) -> Result<ClientHealthResponse> {
    let client = http::build_client(config)?;
    let ts = utils::current_timestamp();
    let url = format!(
        "{}/dna/intent/api/v1/client-health?timestamp={}",
        config.dnac_url, ts
    );
    http::get_authenticated(&client, config, token, &url).await
}
