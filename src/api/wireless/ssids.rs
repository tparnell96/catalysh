// src/api/wireless/ssids.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ssid {
    pub ssid: Option<String>,
    pub security_level: Option<String>,
    pub passphrase: Option<String>,
    pub enable_broadcast_ssid: Option<bool>,
    pub fast_lane: Option<bool>,
    pub enable_mac_filtering: Option<bool>,
    pub traffic_type: Option<String>,
    pub radio_policy: Option<String>,
    pub name: Option<String>,
    pub wlan_type: Option<String>,
    pub auth_type: Option<String>,
    pub id: Option<String>,
    pub inherited_site_id: Option<String>,
    pub inherited_site_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SsidListResponse {
    pub response: Option<Vec<Ssid>>,
    pub version: Option<String>,
}

pub async fn get_ssids(
    config: &Config,
    token: &Token,
    site_id: &str,
) -> Result<SsidListResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/sites/{}/wirelessSettings/ssids",
        config.dnac_url, site_id
    );
    http::get_authenticated(&client, config, token, &url).await
}
