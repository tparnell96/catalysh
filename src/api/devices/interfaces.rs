// src/api/devices/interfaces.rs
// Device interface list and per-interface neighbor (CDP/LLDP) queries.

#![allow(dead_code)]

use crate::api::authentication::auth::Token;
use crate::api::topology;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Interface list ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub neighbor_ip: Option<String>,
    pub neighbor_platform_id: Option<String>,
    pub neighbor_role: Option<String>,
    pub neighbor_interface: Option<NeighborInterface>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborInterface {
    pub status: Option<String>,
    pub admin_status: Option<String>,
    pub description: Option<String>,
    pub vlan_id: Option<String>,
    pub native_vlan_id: Option<String>,
    pub speed: Option<String>,
    pub duplex: Option<String>,
    pub media_type: Option<String>,
    pub mac_address: Option<String>,
}

impl From<&DeviceInterface> for NeighborInterface {
    fn from(interface: &DeviceInterface) -> Self {
        Self {
            status: interface.status.clone(),
            admin_status: interface.admin_status.clone(),
            description: interface.description.clone(),
            vlan_id: interface.vlan_id.clone(),
            native_vlan_id: interface.native_vlan_id.clone(),
            speed: interface.speed.clone(),
            duplex: interface.duplex.clone(),
            media_type: interface.media_type.clone(),
            mac_address: interface.mac_address.clone(),
        }
    }
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
                    neighbor_ip: None,
                    neighbor_platform_id: None,
                    neighbor_role: None,
                    neighbor_interface: None,
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
    get_neighbors_from_topology_internal(config, token, hostname, mgmt_ip, false).await
}

/// Look up an AP uplink from physical topology and enrich the connected switch
/// port with interface inventory data.
pub async fn get_ap_neighbors_from_topology(
    config: &Config,
    token: &Token,
    hostname: &str,
    mgmt_ip: &str,
) -> Result<Vec<DeviceNeighbor>> {
    get_neighbors_from_topology_internal(config, token, hostname, mgmt_ip, true).await
}

async fn get_neighbors_from_topology_internal(
    config: &Config,
    token: &Token,
    hostname: &str,
    mgmt_ip: &str,
    enrich_interfaces: bool,
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

    let target_ids = node_ids(target);
    let node_by_id: HashMap<&str, &topology::TopologyNode> = nodes
        .iter()
        .flat_map(|node| node_ids(node).into_iter().map(move |id| (id, node)))
        .collect();

    let mut results = Vec::new();

    for link in &links {
        let src = link.source.as_deref().unwrap_or("");
        let tgt = link.target.as_deref().unwrap_or("");

        let (peer_id, local_port, peer_port) = if target_ids.contains(&src) {
            (
                tgt,
                topology_port(link, true, true),
                topology_port(link, false, false),
            )
        } else if target_ids.contains(&tgt) {
            (
                src,
                topology_port(link, false, true),
                topology_port(link, true, false),
            )
        } else {
            continue;
        };

        let peer = node_by_id.get(peer_id).copied();
        let peer_label = peer.and_then(|n| n.label.as_deref()).unwrap_or(peer_id);
        let peer_role = peer.and_then(|n| n.role.as_deref()).unwrap_or("unknown");
        let status = link.link_status.as_deref().unwrap_or("unknown").to_string();

        let mut neighbor = DeviceNeighbor {
            local_port: local_port.unwrap_or_else(|| "—".to_string()),
            port_status: status,
            neighbor_device: format!("{} ({})", peer_label, peer_role),
            neighbor_port: peer_port.unwrap_or_else(|| "—".to_string()),
            capabilities: vec![],
            neighbor_ip: peer.and_then(|n| n.ip.clone()),
            neighbor_platform_id: peer.and_then(|n| n.platform_id.clone()),
            neighbor_role: peer.and_then(|n| n.role.clone()),
            neighbor_interface: None,
        };

        if let (true, Some(peer_uuid), false) = (
            enrich_interfaces,
            peer.and_then(|n| n.id.as_deref()),
            neighbor.neighbor_port == "—",
        ) {
            match get_device_interfaces(config, token, peer_uuid).await {
                Ok(interfaces) => {
                    neighbor.neighbor_interface = interfaces
                        .iter()
                        .find(|interface| {
                            interface.port_name.as_deref().is_some_and(|name| {
                                name.eq_ignore_ascii_case(&neighbor.neighbor_port)
                            })
                        })
                        .map(NeighborInterface::from);
                }
                Err(error) => debug!(
                    "Could not enrich topology neighbor {} interface {}: {}",
                    peer_label, neighbor.neighbor_port, error
                ),
            }
        }

        results.push(neighbor);
    }

    Ok(results)
}

fn node_ids(node: &topology::TopologyNode) -> Vec<&str> {
    [node.id.as_deref(), node.data_path_id.as_deref()]
        .into_iter()
        .flatten()
        .collect()
}

fn topology_port(
    link: &topology::TopologyLink,
    source_end: bool,
    local_fallback: bool,
) -> Option<String> {
    let direct = if source_end {
        link.start_port_name.clone()
    } else {
        link.end_port_name.clone()
    };
    let endpoint_key = if source_end {
        "sourceInterfaceName"
    } else {
        "targetInterfaceName"
    };
    let legacy_key = if local_fallback {
        "localInterface"
    } else {
        "remoteInterface"
    };

    direct
        .or_else(|| extract_str(&link.interface_details, endpoint_key))
        .or_else(|| extract_str(&link.interface_details, legacy_key))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, label: &str) -> topology::TopologyNode {
        topology::TopologyNode {
            label: Some(label.to_string()),
            ip: None,
            device_type: None,
            role: None,
            family: None,
            id: Some(id.to_string()),
            network_type: None,
            os: None,
            platform_id: None,
            data_path_id: None,
            upper_node: None,
            node_type: None,
            order: None,
            additional_info: None,
            custom_param: None,
        }
    }

    #[test]
    fn deserializes_documented_topology_port_names() {
        let link: topology::TopologyLink = serde_json::from_value(serde_json::json!({
            "source": "switch-id",
            "target": "ap-id",
            "startPortName": "GigabitEthernet1/0/24",
            "endPortName": "GigabitEthernet0",
            "linkStatus": "up"
        }))
        .unwrap();

        assert_eq!(
            topology_port(&link, true, true).as_deref(),
            Some("GigabitEthernet1/0/24")
        );
        assert_eq!(
            topology_port(&link, false, false).as_deref(),
            Some("GigabitEthernet0")
        );
    }

    #[test]
    fn indexes_both_inventory_and_data_path_ids() {
        let mut ap = node("inventory-id", "AP01");
        ap.data_path_id = Some("data-path-id".to_string());

        assert_eq!(node_ids(&ap), vec!["inventory-id", "data-path-id"]);
    }

    #[test]
    fn builds_interface_enrichment_from_inventory_data() {
        let interface = DeviceInterface {
            id: None,
            instance_uuid: None,
            device_id: None,
            port_name: Some("GigabitEthernet1/0/24".to_string()),
            port_type: None,
            interface_type: None,
            admin_status: Some("UP".to_string()),
            status: Some("up".to_string()),
            mac_address: Some("00:11:22:33:44:55".to_string()),
            ipv4_address: None,
            vlan_id: Some("20".to_string()),
            native_vlan_id: Some("20".to_string()),
            speed: Some("1000000000".to_string()),
            duplex: Some("FullDuplex".to_string()),
            media_type: Some("1000BaseT".to_string()),
            description: Some("Lobby AP".to_string()),
            series: None,
            pid: None,
        };

        let details = NeighborInterface::from(&interface);
        assert_eq!(details.status.as_deref(), Some("up"));
        assert_eq!(details.vlan_id.as_deref(), Some("20"));
        assert_eq!(details.description.as_deref(), Some("Lobby AP"));
    }
}
