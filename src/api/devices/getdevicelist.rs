use crate::api::authentication::auth::{self, Token};
use crate::config::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
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
    pub uptime_seconds: Option<i64>, // Assuming "integer" corresponds to i64
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

#[derive(Debug, Deserialize)]
struct DevicesResponse {
    response: Vec<AllDevices>,
}

pub async fn get_all_devices(config: &Config, token: &Token) -> Result<Vec<AllDevices>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let mut all_devices: Vec<AllDevices> = Vec::new();
    let mut offset = 1; // Adjust based on API documentation (could be 0)
    let limit = 500;    // Set the limit as per API maximum

    loop {
        let devices_url = format!(
            "{}/dna/intent/api/v1/network-device?offset={}&limit={}",
            config.dnac_url, offset, limit
        );

        // Perform the API request with reauthentication handling
        let devices_response: DevicesResponse =
            send_authenticated_request(&client, config, token, &devices_url).await?;

        let devices = devices_response.response;

        if devices.is_empty() {
            // No more devices to fetch
            break;
        }

        all_devices.extend(devices);

        // Increment offset
        offset += limit;
    }

    Ok(all_devices)
}

/// Sends an authenticated GET request, handling reauthentication if necessary.
async fn send_authenticated_request<T: serde::de::DeserializeOwned>(
    client: &Client,
    config: &Config,
    token: &Token,
    url: &str,
) -> Result<T> {
    let mut current_token = token.clone();

    loop {
        let resp = client
            .get(url)
            .header("X-Auth-Token", &current_token.value)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            // Token expired or invalid, reauthenticate and retry
            eprintln!("Token expired. Reauthenticating...");
            current_token = auth::authenticate(config).await?;
            continue; // Retry the request with the new token
        }

        if !resp.status().is_success() {
            return Err(anyhow!(
                "Failed to complete request: {}",
                resp.status()
            ));
        }

        // Deserialize and return the response
        let result = resp.json::<T>().await?;
        return Ok(result);
    }
}
