#![allow(dead_code)]
// src/api/topology/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyNode {
    pub label: Option<String>,
    pub ip: Option<String>,
    pub device_type: Option<String>,
    pub role: Option<String>,
    pub family: Option<String>,
    pub id: Option<String>,
    pub network_type: Option<String>,
    pub os: Option<String>,
    pub platform_id: Option<String>,
    pub data_path_id: Option<String>,
    pub upper_node: Option<String>,
    pub node_type: Option<String>,
    pub order: Option<i64>,
    pub additional_info: Option<serde_json::Value>,
    pub custom_param: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopologyLink {
    pub source: Option<String>,
    pub target: Option<String>,
    pub link_status: Option<String>,
    pub label: Option<serde_json::Value>,
    pub id: Option<String>,
    pub source_data_path_id: Option<String>,
    pub target_data_path_id: Option<String>,
    pub port_utilization: Option<serde_json::Value>,
    pub interface_details: Option<serde_json::Value>,
    pub additional_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopologyGraph {
    pub nodes: Option<Vec<TopologyNode>>,
    pub links: Option<Vec<TopologyLink>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalTopologyResponse {
    pub response: Option<TopologyGraph>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteTopologyNode {
    pub name: Option<String>,
    pub id: Option<String>,
    pub location_address: Option<String>,
    pub parent_id: Option<String>,
    pub site_name_hierarchy: Option<String>,
    pub location_type: Option<String>,
    pub site_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteTopologySites {
    pub sites: Option<Vec<SiteTopologyNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteTopologyResponse {
    pub response: Option<SiteTopologySites>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VlanNamesResponse {
    pub response: Option<Vec<String>>,
}

pub async fn get_physical_topology(
    config: &Config,
    token: &Token,
) -> Result<PhysicalTopologyResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/topology/physical-topology",
        config.dnac_url
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_site_topology(config: &Config, token: &Token) -> Result<SiteTopologyResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/topology/site-topology",
        config.dnac_url
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_l3_topology(
    config: &Config,
    token: &Token,
    topology_type: &str,
) -> Result<PhysicalTopologyResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/topology/l3/{}",
        config.dnac_url, topology_type
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_vlan_names(config: &Config, token: &Token) -> Result<VlanNamesResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/topology/vlan/vlan-names",
        config.dnac_url
    );
    http::get_authenticated(&client, config, token, &url).await
}

/// POST helper for this module
pub async fn post_authenticated<T>(
    config: &Config,
    token: &Token,
    url: &str,
    body: &serde_json::Value,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let client = http::build_client(config)?;
    let resp = client
        .post(url)
        .header("X-Auth-Token", &token.value)
        .json(body)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("POST to {} failed: {}", url, resp.status()));
    }

    Ok(resp.json::<T>().await?)
}
