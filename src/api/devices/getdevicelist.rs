use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AllDevices {
    pub reachability_failure_reason: Option<String>,
    pub reachability_status: Option<String>,
    pub series: Option<String>,
    pub snmp_contact: Option<String>,
    pub snmp_location: Option<String>,
    pub tag_count: Option<String>,
    pub tunnel_udp_port: Option<serde_json::Value>, // Use `serde_json::Value` for fields with undefined schema
    pub uptime_seconds: Option<i64>,                // Assuming "integer" corresponds to i64
    pub waas_device_mode: Option<serde_json::Value>,
    pub serial_number: Option<String>,
    pub last_update_time: Option<i64>,
    pub mac_address: Option<String>,
    pub up_time: Option<String>,
    pub device_support_level: Option<String>,
    pub hostname: Option<String>,
    pub device_type: Option<String>, // Renamed "type" to "device_type" to avoid reserved keyword
    pub memory_size: Option<String>,
    pub family: Option<String>,
    pub error_code: Option<String>,
    pub software_type: Option<String>,
    pub software_version: Option<String>,
    pub description: Option<String>,
    pub role_source: Option<String>,
    pub location: Option<serde_json::Value>,
    pub role: Option<String>,
    pub collection_interval: Option<String>,
    pub inventory_status_detail: Option<String>,
    pub ap_ethernet_mac_address: Option<String>,
    pub ap_manager_interface_ip: Option<String>,
    pub associated_wlc_ip: Option<String>,
    pub boot_date_time: Option<String>,
    pub collection_status: Option<String>,
    pub error_description: Option<String>,
    pub interface_count: Option<String>,
    pub last_updated: Option<String>,
    pub line_card_count: Option<String>,
    pub line_card_id: Option<String>,
    pub location_name: Option<serde_json::Value>,
    pub managed_atleast_once: Option<bool>,
    pub management_ip_address: Option<String>,
    pub platform_id: Option<String>,
    pub management_state: Option<String>,
    pub instance_tenant_id: Option<String>,
    pub instance_uuid: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DevicesResponse {
    response: Vec<AllDevices>,
}

pub async fn get_all_devices(config: &Config, token: &Token) -> Result<Vec<AllDevices>> {
    let client = http::build_client(config)?;

    let mut all_devices: Vec<AllDevices> = Vec::new();
    let mut offset = 1;
    let limit = 500; // Set the limit as per API maximum

    loop {
        let devices_url = format!(
            "{}/dna/intent/api/v1/network-device?offset={}&limit={}",
            config.dnac_url, offset, limit
        );

        let devices_response: DevicesResponse =
            http::get_authenticated(&client, config, token, &devices_url).await?;

        let devices = devices_response.response;

        if devices.is_empty() {
            break;
        }

        all_devices.extend(devices);
        offset += limit;
    }

    Ok(all_devices)
}
