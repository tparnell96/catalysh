#![allow(dead_code)]
// src/api/networksettings/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSetting {
    pub instance_type: Option<String>,
    pub key: Option<String>,
    pub value: Option<serde_json::Value>,
    pub instance_uuid: Option<String>,
    pub namespace: Option<String>,
    pub r#type: Option<String>,
    pub group_uuid: Option<String>,
    pub inherited_group_uuid: Option<String>,
    pub inherited_group_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSettingsResponse {
    pub response: Option<Vec<NetworkSetting>>,
    pub version: Option<String>,
}

pub async fn get_global_network_settings(
    config: &Config,
    token: &Token,
) -> Result<NetworkSettingsResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/network", config.dnac_url);
    http::get_authenticated(&client, config, token, &url).await
}
