#![allow(dead_code)]
// src/api/devices/devicecount.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCountResponse {
    pub response: Option<serde_json::Value>,
    pub version: Option<String>,
}

pub async fn get_device_count(config: &Config, token: &Token) -> Result<DeviceCountResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/network-device/count", config.dnac_url);
    http::get_authenticated(&client, config, token, &url).await
}
