#![allow(dead_code)]

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const AP_PROVISION_LEGACY_PATH: &str = "/dna/intent/api/v1/wireless/ap-provision";
const AP_PROVISION_V2_PATH: &str = "/dna/intent/api/v1/wirelessAccessPoints/provision";
const AP_PROVISION_STATUS_PATH: &str =
    "/dna/intent/api/v1/wirelessControllers/{networkDeviceId}/provisionStatus";
const AP_FACTORY_RESET_PATH: &str =
    "/dna/intent/api/v1/wirelessAccessPoints/factoryResetRequest/provision";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProvisionRequest {
    pub ap_zone_name: String,
    pub ap_name: String,
    pub rf_profile: String,
    pub site_name_hierarchy: String,
    pub mac_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProvisionResponse {
    pub response: Option<ApProvisionTaskResponse>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProvisionTaskResponse {
    pub task_id: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApProvisionStatusResponse {
    pub response: Option<ApProvisionStatus>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApProvisionStatus {
    pub status: Option<String>,
    pub provision_details: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactoryResetRequest {
    pub ap_mac_addresses: Vec<String>,
    pub keep_static_ip_config: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FactoryResetResponse {
    pub response: Option<ApProvisionTaskResponse>,
    pub version: Option<String>,
}

pub async fn provision_ap(
    config: &Config,
    token: &Token,
    mac: &str,
    site_hierarchy: &str,
    rf_profile: &str,
    ap_name: &str,
) -> Result<String> {
    let request = ApProvisionRequest {
        ap_zone_name: String::new(),
        ap_name: ap_name.to_string(),
        rf_profile: rf_profile.to_string(),
        site_name_hierarchy: site_hierarchy.to_string(),
        mac_address: mac.to_string(),
    };

    let response = post_json::<_, ApProvisionResponse>(config, token, AP_PROVISION_V2_PATH, &request).await?;
    extract_task_id(response.response, "AP provision")
}

pub async fn get_provision_status(
    config: &Config,
    token: &Token,
    controller_device_id: &str,
) -> Result<ApProvisionStatusResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}{}",
        config.dnac_url,
        AP_PROVISION_STATUS_PATH.replace("{networkDeviceId}", controller_device_id)
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn factory_reset_ap(
    config: &Config,
    token: &Token,
    mac_addresses: Vec<String>,
) -> Result<String> {
    let request = FactoryResetRequest {
        ap_mac_addresses: mac_addresses,
        keep_static_ip_config: false,
    };

    let response =
        post_json::<_, FactoryResetResponse>(config, token, AP_FACTORY_RESET_PATH, &request).await?;
    extract_task_id(response.response, "AP factory reset")
}

async fn post_json<B, R>(config: &Config, token: &Token, path: &str, body: &B) -> Result<R>
where
    B: Serialize + ?Sized,
    R: for<'de> Deserialize<'de>,
{
    let client = http::build_client(config)?;
    let url = format!("{}{}", config.dnac_url, path);
    let resp = client
        .post(&url)
        .header("X-Auth-Token", &token.value)
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("POST to {} failed: {}", url, resp.status()));
    }

    Ok(resp.json::<R>().await?)
}

fn extract_task_id(response: Option<ApProvisionTaskResponse>, operation: &str) -> Result<String> {
    response
        .and_then(|resp| resp.task_id)
        .ok_or_else(|| anyhow!("{} submitted but no task ID was returned", operation))
}
