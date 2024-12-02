// src/api/clients/getclientenrichment.rs

use crate::app::config::Config;
use crate::api::authentication::auth::Token;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ClientEnrichmentResponse(pub Vec<ClientEnrichment>);

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ClientEnrichment {
    pub userDetails: Option<UserDetails>,
    pub connectedDevice: Option<Vec<ConnectedDevice>>,
    pub issueDetails: Option<IssueDetails>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct UserDetails {
    pub id: Option<String>,
    pub connectionStatus: Option<String>,
    pub hostType: Option<String>,
    pub userId: Option<String>,
    pub hostName: Option<String>,
    pub hostOs: Option<String>,
    pub hostVersion: Option<String>,
    pub subType: Option<String>,
    pub lastUpdated: Option<i64>,
    pub healthScore: Option<Vec<HealthScore>>,
    pub hostMac: Option<String>,
    pub hostIpV4: Option<String>,
    pub hostIpV6: Option<Vec<String>>,
    pub authType: Option<String>,
    pub vlanId: Option<String>,
    pub ssid: Option<String>,
    pub location: Option<String>,
    pub clientConnection: Option<String>,
    pub connectedDevice: Option<Vec<String>>, // Adjust as per actual data
    pub issueCount: Option<f64>,
    pub rssi: Option<String>,
    pub snr: Option<String>,
    pub dataRate: Option<String>,
    pub port: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct HealthScore {
    pub healthType: Option<String>,
    pub reason: Option<String>,
    pub score: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ConnectedDevice {
    pub deviceDetails: Option<DeviceDetails>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct DeviceDetails {
    pub family: Option<String>,
    pub type_field: Option<String>,
    pub location: Option<String>,
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
    pub tunnelUdpPort: Option<String>,
    pub waasDeviceMode: Option<String>,
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
    pub locationName: Option<String>,
    pub tagCount: Option<String>,
    pub lastUpdated: Option<String>,
    pub instanceUuid: Option<String>,
    pub id: Option<String>,
    pub neighborTopology: Option<Vec<NeighborTopology>>,
    pub cisco360view: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct NeighborTopology {
    pub nodes: Option<Vec<TopologyNode>>,
    pub links: Option<Vec<TopologyLink>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
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
    pub userId: Option<String>,
    pub nodeType: Option<String>,
    pub radioFrequency: Option<String>,
    pub clients: Option<f64>,
    pub count: Option<f64>,
    pub healthScore: Option<f64>,
    pub level: Option<f64>,
    pub fabricGroup: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TopologyLink {
    pub source: Option<String>,
    pub linkStatus: Option<String>,
    pub label: Option<Vec<String>>,
    pub target: Option<String>,
    pub id: Option<String>,
    pub portUtilization: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct IssueDetails {
    pub issue: Option<Vec<Issue>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Issue {
    pub issueId: Option<String>,
    pub issueSource: Option<String>,
    pub issueCategory: Option<String>,
    pub issueName: Option<String>,
    pub issueDescription: Option<String>,
    pub issueEntity: Option<String>,
    pub issueEntityValue: Option<String>,
    pub issueSeverity: Option<String>,
    pub issuePriority: Option<String>,
    pub issueSummary: Option<String>,
    pub issueTimestamp: Option<i64>,
    pub suggestedActions: Option<Vec<SuggestedAction>>,
    pub impactedHosts: Option<Vec<ImpactedHost>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct SuggestedAction {
    pub message: Option<String>,
    pub steps: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ImpactedHost {
    pub hostType: Option<String>,
    pub hostName: Option<String>,
    pub hostOs: Option<String>,
    pub ssid: Option<String>,
    pub connectedInterface: Option<String>,
    pub macAddress: Option<String>,
    pub failedAttempts: Option<i32>,
    pub location: Option<ImpactedHostLocation>,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ImpactedHostLocation {
    pub siteId: Option<String>,
    pub siteType: Option<String>,
    pub area: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub apsImpacted: Option<Vec<String>>,
}

pub async fn get_client_enrichment(
    config: &Config,
    token: &Token,
    entity_type: &str,
    entity_value: &str,
    issue_category: Option<&str>,
) -> Result<ClientEnrichmentResponse> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}/dna/intent/api/v1/client-enrichment-details", config.dnac_url);

    let mut query_params = HashMap::new();
    query_params.insert("entity_type", entity_type);
    query_params.insert("entity_value", entity_value);
    if let Some(category) = issue_category {
        query_params.insert("issueCategory", category);
    }

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&query_params)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve client enrichment details: {}",
            resp.status()
        ));
    }

    let enrichment_response = resp.json::<ClientEnrichmentResponse>().await?;
    Ok(enrichment_response)
}
