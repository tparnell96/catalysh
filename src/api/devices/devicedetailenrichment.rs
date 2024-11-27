// src/api/devices/devicedetailenrichment.rs

use crate::app::config::Config;
use crate::api::authentication::auth::Token;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceEnrichmentResponse {
    pub deviceDetails: DeviceDetails,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct DeviceDetails {
    pub family: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub location: Option<serde_json::Value>,
    pub errorCode: Option<String>,
    pub macAddress: Option<String>,
    pub role: Option<String>,
    pub apManagerInterfaceIp: Option<String>,
    pub associatedWlcIp: Option<String>,
    pub bootDateTime: Option<String>,
    pub collectionStatus: Option<String>,
    pub interfaceCount: Option<String>,
    pub lineCardCount: Option<String>,
    pub lineCardId: Option<String>,
    pub managementIpAddress: Option<String>,
    pub memorySize: Option<String>,
    pub platformId: Option<String>,
    pub reachabilityFailureReason: Option<String>,
    pub reachabilityStatus: Option<String>,
    pub snmpContact: Option<String>,
    pub snmpLocation: Option<String>,
    pub tunnelUdpPort: Option<serde_json::Value>,
    pub waasDeviceMode: Option<serde_json::Value>,
    pub series: Option<String>,
    pub inventoryStatusDetail: Option<String>,
    pub collectionInterval: Option<String>,
    pub serialNumber: Option<String>,
    pub softwareVersion: Option<String>,
    pub roleSource: Option<String>,
    pub hostname: Option<String>,
    pub upTime: Option<String>,
    pub lastUpdateTime: Option<i64>,
    pub errorDescription: Option<String>,
    pub locationName: Option<serde_json::Value>,
    pub tagCount: Option<String>,
    pub lastUpdated: Option<String>,
    pub instanceUuid: Option<String>,
    pub id: Option<String>,
    pub neighborTopology: Option<Vec<NeighborTopology>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct NeighborTopology {
    pub nodes: Option<Vec<TopologyNode>>,
    pub links: Option<Vec<TopologyLink>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TopologyNode {
    pub role: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub deviceType: Option<String>,
    pub platformId: Option<String>,
    pub family: Option<String>,
    pub ip: Option<String>,
    pub softwareVersion: Option<String>,
    pub userId: Option<serde_json::Value>,
    pub nodeType: Option<String>,
    pub radioFrequency: Option<serde_json::Value>,
    pub clients: Option<serde_json::Value>,
    pub count: Option<serde_json::Value>,
    pub healthScore: Option<i32>,
    pub level: Option<f64>,
    pub fabricGroup: Option<serde_json::Value>,
    pub connectedDevice: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TopologyLink {
    pub source: Option<String>,
    pub linkStatus: Option<String>,
    pub label: Option<Vec<String>>,
    pub target: Option<String>,
    pub id: Option<serde_json::Value>,
    pub portUtilization: Option<serde_json::Value>,
}

pub async fn get_device_enrichment(
    config: &Config,
    token: &Token,
    entity_type: &str,
    entity_value: &str,
) -> Result<DeviceDetails> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}/dna/intent/api/v1/device-enrichment-details", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .header("entity_type", entity_type)
        .header("entity_value", entity_value)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve device enrichment details: {}",
            resp.status()
        ));
    }

    let enrichment_responses = resp.json::<Vec<DeviceEnrichmentResponse>>().await?;

    if let Some(first_response) = enrichment_responses.into_iter().next() {
        Ok(first_response.deviceDetails)
    } else {
        Err(anyhow!("No device enrichment details found"))
    }
}
