// src/api/wireless/getaccesspointconfig.rs

use crate::app::config::Config;
use crate::api::authentication::auth::Token;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ApConfig {
    pub instanceUuid: Option<serde_json::Value>,
    pub instanceId: Option<f64>,
    pub authEntityId: Option<serde_json::Value>,
    pub displayName: Option<String>,
    pub authEntityClass: Option<serde_json::Value>,
    pub instanceTenantId: Option<String>,
    pub _orderedListOEIndex: Option<f64>,
    pub _orderedListOEAssocName: Option<serde_json::Value>,
    pub _creationOrderIndex: Option<f64>,
    pub _isBeingChanged: Option<bool>,
    pub deployPending: Option<String>,
    pub instanceCreatedOn: Option<serde_json::Value>,
    pub instanceUpdatedOn: Option<serde_json::Value>,
    pub changeLogList: Option<serde_json::Value>,
    pub instanceOrigin: Option<serde_json::Value>,
    pub lazyLoadedEntities: Option<serde_json::Value>,
    pub instanceVersion: Option<f64>,
    pub adminStatus: Option<String>,
    pub apHeight: Option<f64>,
    pub apMode: Option<String>,
    pub apName: Option<String>,
    pub ethMac: Option<String>,
    pub failoverPriority: Option<String>,
    pub ledBrightnessLevel: Option<i64>,
    pub ledStatus: Option<String>,
    pub location: Option<String>,
    pub macAddress: Option<String>,
    pub primaryControllerName: Option<String>,
    pub primaryIpAddress: Option<String>,
    pub secondaryControllerName: Option<String>,
    pub secondaryIpAddress: Option<String>,
    pub tertiaryControllerName: Option<String>,
    pub tertiaryIpAddress: Option<String>,
    pub meshDTOs: Option<Vec<serde_json::Value>>,
    pub radioDTOs: Option<Vec<RadioDTO>>,
    pub internalKey: Option<InternalKey>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct RadioDTO {
    pub instanceUuid: Option<serde_json::Value>,
    pub instanceId: Option<f64>,
    pub authEntityId: Option<serde_json::Value>,
    pub displayName: Option<String>,
    pub authEntityClass: Option<serde_json::Value>,
    pub instanceTenantId: Option<String>,
    pub _orderedListOEIndex: Option<f64>,
    pub _orderedListOEAssocName: Option<serde_json::Value>,
    pub _creationOrderIndex: Option<f64>,
    pub _isBeingChanged: Option<bool>,
    pub deployPending: Option<String>,
    pub instanceCreatedOn: Option<serde_json::Value>,
    pub instanceUpdatedOn: Option<serde_json::Value>,
    pub changeLogList: Option<serde_json::Value>,
    pub instanceOrigin: Option<serde_json::Value>,
    pub lazyLoadedEntities: Option<serde_json::Value>,
    pub instanceVersion: Option<f64>,
    pub adminStatus: Option<String>,
    pub antennaAngle: Option<f64>,
    pub antennaElevAngle: Option<f64>,
    pub antennaGain: Option<i64>,
    pub antennaPatternName: Option<String>,
    pub channelAssignmentMode: Option<String>,
    pub channelNumber: Option<i64>,
    pub channelWidth: Option<String>,
    pub cleanAirSI: Option<String>,
    pub ifType: Option<i64>,
    pub ifTypeValue: Option<String>,
    pub macAddress: Option<String>,
    pub powerAssignmentMode: Option<String>,
    pub powerlevel: Option<i64>,
    pub radioBand: Option<serde_json::Value>,
    pub radioRoleAssignment: Option<serde_json::Value>,
    pub slotId: Option<i64>,
    pub internalKey: Option<InternalKey>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct InternalKey {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub id: Option<f64>,
    pub longType: Option<String>,
    pub url: Option<String>,
}

pub async fn get_ap_config(
    config: &Config,
    token: &Token,
    mac_address: &str,
) -> Result<ApConfig> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}/dna/intent/api/v1/wireless/accesspoint-configuration/summary", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&[("key", mac_address)])
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve AP config: {}",
            resp.status()
        ));
    }

    let ap_config = resp.json::<ApConfig>().await?;
    Ok(ap_config)
}
