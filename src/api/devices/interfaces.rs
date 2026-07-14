// src/api/devices/interfaces.rs
// Device interface list and per-interface neighbor (CDP/LLDP) queries.

#![allow(dead_code)]

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// ── Interface list ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInterface {
    pub id: Option<String>,
    pub instance_uuid: Option<String>,
    pub device_id: Option<String>,
    pub port_name: Option<String>,
    pub port_type: Option<String>,
    pub interface_type: Option<String>,
    pub admin_status: Option<String>,
    pub status: Option<String>,
    pub mac_address: Option<String>,
    pub ipv4_address: Option<String>,
    pub vlan_id: Option<String>,
    pub native_vlan_id: Option<String>,
    pub speed: Option<String>,
    pub duplex: Option<String>,
    pub media_type: Option<String>,
    pub description: Option<String>,
    pub series: Option<String>,
    pub pid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InterfaceListResponse {
    response: Vec<DeviceInterface>,
}

/// Fetch all interfaces for a device by its UUID.
pub async fn get_device_interfaces(
    config: &Config,
    token: &Token,
    device_id: &str,
) -> Result<Vec<DeviceInterface>> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/interface/network-device/{}",
        config.dnac_url, device_id
    );
    let resp: InterfaceListResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response)
}

// ── Neighbor / CDP-LLDP detail ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighborDetail {
    pub neighbor_device: Option<String>,
    pub neighbor_port: Option<String>,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NeighborResponse {
    response: Option<NeighborDetail>,
}

/// Query the CDP/LLDP neighbor for a single interface.
/// Returns `None` if the interface has no discovered neighbor.
pub async fn get_interface_neighbor(
    config: &Config,
    token: &Token,
    device_uuid: &str,
    interface_uuid: &str,
) -> Result<Option<NeighborDetail>> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/network-device/{}/interface/{}/neighbor",
        config.dnac_url, device_uuid, interface_uuid
    );
    let resp: NeighborResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response)
}

// ── Combined AP uplink helper ────────────────────────────────────────────────

/// One row in the AP uplink table — one physical interface + its neighbor.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApUplinkNeighbor {
    pub ap_port: String,
    pub ap_port_status: String,
    pub neighbor_device: String,
    pub neighbor_port: String,
    pub capabilities: Vec<String>,
}

/// Fetch all interfaces for `device_uuid` and query the neighbor for each
/// physical uplink interface.  Returns a vec of (AP port → neighbor) entries.
pub async fn get_ap_uplink_neighbors(
    config: &Config,
    token: &Token,
    device_uuid: &str,
) -> Result<Vec<ApUplinkNeighbor>> {
    let interfaces = get_device_interfaces(config, token, device_uuid).await?;

    let mut results = Vec::new();

    for iface in &interfaces {
        let iface_id = match iface.id.as_deref().or(iface.instance_uuid.as_deref()) {
            Some(id) => id,
            None => continue,
        };
        let port_name = iface.port_name.as_deref().unwrap_or("unknown").to_string();

        // Skip virtual / loopback / tunnel interfaces
        let port_type = iface.port_type.as_deref().unwrap_or("");
        if port_type.contains("Virtual") || port_type.contains("Loopback") {
            continue;
        }
        let iface_type = iface.interface_type.as_deref().unwrap_or("");
        if iface_type == "VIRTUAL" {
            continue;
        }

        match get_interface_neighbor(config, token, device_uuid, iface_id).await {
            Ok(Some(neighbor)) => {
                results.push(ApUplinkNeighbor {
                    ap_port: port_name,
                    ap_port_status: iface.status.as_deref().unwrap_or("unknown").to_string(),
                    neighbor_device: neighbor.neighbor_device.unwrap_or_else(|| "—".to_string()),
                    neighbor_port: neighbor.neighbor_port.unwrap_or_else(|| "—".to_string()),
                    capabilities: neighbor.capabilities.unwrap_or_default(),
                });
            }
            Ok(None) => {} // no neighbor on this port, skip
            Err(_) => {}   // 404 / no data, skip
        }
    }

    Ok(results)
}
