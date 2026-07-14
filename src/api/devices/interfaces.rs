// src/api/devices/interfaces.rs
// Device interface list and per-interface neighbor (CDP/LLDP) queries.

#![allow(dead_code)]

use crate::api::authentication::auth::Token;
use crate::api::topology;
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
    let resp: InterfaceListResponse = http::get_authenticated(&client, config, token, &url).await?;
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
    let resp: NeighborResponse = http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response)
}

// ── Combined AP uplink helper ────────────────────────────────────────────────

/// One row in the neighbor table — one physical interface + its CDP/LLDP neighbor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceNeighbor {
    pub local_port: String,
    pub port_status: String,
    pub neighbor_device: String,
    pub neighbor_port: String,
    pub capabilities: Vec<String>,
}

/// Fetch all interfaces for `device_uuid` and query the CDP/LLDP neighbor for
/// each physical port.  Skips virtual/loopback/tunnel interfaces.
pub async fn get_device_neighbors(
    config: &Config,
    token: &Token,
    device_uuid: &str,
) -> Result<Vec<DeviceNeighbor>> {
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
        if iface.interface_type.as_deref() == Some("VIRTUAL") {
            continue;
        }

        match get_interface_neighbor(config, token, device_uuid, iface_id).await {
            Ok(Some(neighbor)) => {
                results.push(DeviceNeighbor {
                    local_port: port_name,
                    port_status: iface.status.as_deref().unwrap_or("unknown").to_string(),
                    neighbor_device: neighbor.neighbor_device.unwrap_or_else(|| "—".to_string()),
                    neighbor_port: neighbor.neighbor_port.unwrap_or_else(|| "—".to_string()),
                    capabilities: neighbor.capabilities.unwrap_or_default(),
                });
            }
            Ok(None) => {} // no neighbor on this port
            Err(_) => {}   // 404 / no data, skip silently
        }
    }

    Ok(results)
}

// Keep the old name as an alias so existing call sites compile without changes.
#[allow(deprecated)]
pub async fn get_ap_uplink_neighbors(
    config: &Config,
    token: &Token,
    device_uuid: &str,
) -> Result<Vec<DeviceNeighbor>> {
    get_device_neighbors(config, token, device_uuid).await
}

// ── Topology-based neighbor lookup ───────────────────────────────────────────
// Used for APs and any device where the interface management API returns 404.

/// Look up neighbors for a device from the physical topology graph.
/// Matches the device by hostname (`label`) or management IP, then follows
/// links to find every directly connected peer node.
pub async fn get_neighbors_from_topology(
    config: &Config,
    token: &Token,
    hostname: &str,
    mgmt_ip: &str,
) -> Result<Vec<DeviceNeighbor>> {
    let topo = topology::get_physical_topology(config, token).await?;
    let graph = match topo.response {
        Some(g) => g,
        None => return Ok(vec![]),
    };

    let nodes = graph.nodes.unwrap_or_default();
    let links = graph.links.unwrap_or_default();

    // Find the target node by hostname (label) or IP
    let hostname_lower = hostname.to_lowercase();
    let target_node = nodes.iter().find(|n| {
        n.label
            .as_deref()
            .map(|l| l.to_lowercase() == hostname_lower)
            .unwrap_or(false)
            || n.ip.as_deref() == Some(mgmt_ip)
    });

    let target = match target_node {
        Some(n) => n,
        None => return Ok(vec![]),
    };

    // The topology graph uses dataPathId to identify nodes in links.
    // Fall back to `id` if dataPathId is absent.
    let target_dpid = target
        .data_path_id
        .as_deref()
        .or(target.id.as_deref())
        .unwrap_or("");

    // Build a lookup map: dataPathId → node (for resolving the other end)
    use std::collections::HashMap;
    let node_by_dpid: HashMap<&str, &topology::TopologyNode> = nodes
        .iter()
        .filter_map(|n| {
            n.data_path_id
                .as_deref()
                .or(n.id.as_deref())
                .map(|dpid| (dpid, n))
        })
        .collect();

    let mut results = Vec::new();

    for link in &links {
        let src = link.source.as_deref().unwrap_or("");
        let tgt = link.target.as_deref().unwrap_or("");

        // Is our device one end of this link?
        let peer_dpid = if src == target_dpid {
            tgt
        } else if tgt == target_dpid {
            src
        } else {
            continue;
        };

        let peer = node_by_dpid.get(peer_dpid);
        let peer_label = peer.and_then(|n| n.label.as_deref()).unwrap_or(peer_dpid);
        let peer_role = peer.and_then(|n| n.role.as_deref()).unwrap_or("unknown");

        // Port info lives in the link's interface_details or label fields
        // These are serde_json::Value — extract best-effort strings
        let local_port = extract_str(&link.interface_details, "localInterface")
            .or_else(|| extract_str(&link.interface_details, "interfaceName"))
            .unwrap_or_else(|| "—".to_string());
        let peer_port = extract_str(&link.interface_details, "remoteInterface")
            .or_else(|| extract_str(&link.interface_details, "neighborInterfaceName"))
            .unwrap_or_else(|| "—".to_string());
        let status = link.link_status.as_deref().unwrap_or("unknown").to_string();

        results.push(DeviceNeighbor {
            local_port,
            port_status: status,
            neighbor_device: format!("{} ({})", peer_label, peer_role),
            neighbor_port: peer_port,
            capabilities: vec![],
        });
    }

    Ok(results)
}

/// Try to pull a string field out of a serde_json::Value that may be an object
/// or null.
fn extract_str(val: &Option<serde_json::Value>, key: &str) -> Option<String> {
    val.as_ref()?
        .as_object()?
        .get(key)?
        .as_str()
        .map(String::from)
}
