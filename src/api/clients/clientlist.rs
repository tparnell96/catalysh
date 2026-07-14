// src/api/clients/clientlist.rs

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientHealth {
    pub overall_score: Option<i64>,
    pub onboarding_score: Option<i64>,
    pub connected_score: Option<i64>,
    pub link_error_percentage_threshold: Option<f64>,
    pub is_link_error_included: Option<bool>,
    pub rssi_threshold: Option<f64>,
    pub snr_threshold: Option<f64>,
    pub is_rssi_included: Option<bool>,
    pub is_snr_included: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientTraffic {
    pub tx_bytes: Option<i64>,
    pub rx_bytes: Option<i64>,
    pub tx_packets: Option<i64>,
    pub rx_packets: Option<i64>,
    pub tx_rate: Option<f64>,
    pub rx_rate: Option<f64>,
    pub rx_retries: Option<i64>,
    pub rx_retry_pct: Option<f64>,
    pub tx_drops: Option<i64>,
    pub rx_drops: Option<i64>,
    pub dns_request_count: Option<i64>,
    pub dns_response_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectedNetworkDevice {
    pub connected_network_device_id: Option<String>,
    pub connected_network_device_name: Option<String>,
    pub connected_network_device_mac: Option<String>,
    pub interface_name: Option<String>,
    pub interface_speed: Option<i64>,
    pub duplex_mode: Option<String>,
    pub connected_network_device_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnection {
    pub vlan_id: Option<String>,
    pub session_duration: Option<i64>,
    pub vn_id: Option<String>,
    pub l2_vn: Option<String>,
    pub l3_vn: Option<String>,
    pub security_group_tag: Option<String>,
    pub link_speed: Option<f64>,
    pub bridge_v_m_mode: Option<String>,
    pub band: Option<String>,
    pub ssid: Option<String>,
    pub auth_type: Option<String>,
    pub wlc_name: Option<String>,
    pub wlc_id: Option<String>,
    pub ap_mac: Option<String>,
    pub ap_ethernet_mac: Option<String>,
    pub ap_mode: Option<String>,
    pub radio_id: Option<i64>,
    pub channel: Option<String>,
    pub channel_width: Option<String>,
    pub protocol: Option<String>,
    pub protocol_capability: Option<String>,
    pub upn_id: Option<String>,
    pub upn_name: Option<String>,
    pub upn_owner: Option<String>,
    pub upn_duid: Option<String>,
    pub rssi: Option<f64>,
    pub snr: Option<f64>,
    pub data_rate: Option<f64>,
    pub is_ios_analytics_capable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientOnboarding {
    pub avg_run_duration: Option<i64>,
    pub max_run_duration: Option<i64>,
    pub avg_assoc_duration: Option<i64>,
    pub max_assoc_duration: Option<i64>,
    pub avg_auth_duration: Option<i64>,
    pub max_auth_duration: Option<i64>,
    pub avg_dhcp_duration: Option<i64>,
    pub max_dhcp_duration: Option<i64>,
    pub max_roaming_duration: Option<i64>,
    pub aaa_server_ip: Option<String>,
    pub dhcp_server_ip: Option<String>,
    pub onboarding_time: Option<i64>,
    pub auth_done_time: Option<i64>,
    pub assoc_done_time: Option<i64>,
    pub dhcp_done_time: Option<i64>,
    pub roaming_time: Option<i64>,
    pub failed_roaming_count: Option<i64>,
    pub successful_roaming_count: Option<i64>,
    pub total_roaming_attempts: Option<i64>,
    pub assoc_failure_reason: Option<String>,
    pub aaa_failure_reason: Option<String>,
    pub dhcp_failure_reason: Option<String>,
    pub other_failure_reason: Option<String>,
    pub latest_failure_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientLatency {
    pub video: Option<f64>,
    pub voice: Option<f64>,
    pub best_effort: Option<f64>,
    pub background: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientListItem {
    pub id: Option<String>,
    pub mac_address: Option<String>,
    #[serde(rename = "type")]
    pub client_type: Option<String>,
    pub name: Option<String>,
    pub user_id: Option<String>,
    pub ipv4_address: Option<String>,
    pub ipv6_addresses: Option<Vec<String>>,
    pub sub_type: Option<String>,
    pub vendor: Option<String>,
    pub os_type: Option<String>,
    pub os_version: Option<String>,
    pub connected_network_device_name: Option<String>,
    pub connected_network_device_mac: Option<String>,
    pub connection_status: Option<String>,
    pub tracked: Option<String>,
    pub is_private_mac_address: Option<bool>,
    pub health: Option<ClientHealth>,
    pub traffic: Option<ClientTraffic>,
    pub connected_network_device: Option<ConnectedNetworkDevice>,
    pub connection: Option<ClientConnection>,
    pub onboarding: Option<ClientOnboarding>,
    pub latency: Option<ClientLatency>,
    pub site_hierarchy: Option<String>,
    pub site_hierarchy_id: Option<String>,
    pub site_id: Option<String>,
    pub last_updated_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientListPage {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub count: Option<i64>,
    pub sort_by: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientListResponse {
    pub response: Option<Vec<ClientListItem>>,
    pub page: Option<ClientListPage>,
}

#[allow(clippy::too_many_arguments)]
pub async fn get_client_list(
    config: &Config,
    token: &Token,
    mac: Option<&str>,
    ipv4: Option<&str>,
    ipv6: Option<&str>,
    ssid: Option<&str>,
    client_type: Option<&str>,
    site_id: Option<&str>,
    limit: u32,
    offset: u32,
) -> Result<Vec<ClientListItem>> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/data/api/v1/clients", config.dnac_url);

    let mut req = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&[("limit", limit.to_string()), ("offset", offset.to_string())]);

    if let Some(m) = mac {
        req = req.query(&[("macAddress", m)]);
    }
    if let Some(ip) = ipv4 {
        req = req.query(&[("ipv4Address", ip)]);
    }
    if let Some(ip) = ipv6 {
        req = req.query(&[("ipv6Address", ip)]);
    }
    if let Some(s) = ssid {
        req = req.query(&[("ssid", s)]);
    }
    if let Some(t) = client_type {
        req = req.query(&[("type", t)]);
    }
    if let Some(sid) = site_id {
        req = req.query(&[("siteId", sid)]);
    }

    let resp = req.send().await?;

    if !resp.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to retrieve client list: {}",
            resp.status()
        ));
    }

    let list_resp = resp.json::<ClientListResponse>().await?;
    Ok(list_resp.response.unwrap_or_default())
}
