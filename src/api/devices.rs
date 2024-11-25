use crate::auth::{self, Token};
use crate::config::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Device {
    pub hostname: Option<String>,
    pub macAddress: Option<String>,
    pub apEthernetMacAddress: Option<String>,
    pub managementIpAddress: Option<String>,
    pub serialNumber: Option<String>,
    pub associatedWlcIp: Option<String>,
    pub softwareVersion: Option<String>,
    // Add more fields as needed
}

#[derive(Debug, Deserialize)]
struct DevicesResponse {
    response: Vec<Device>,
    // totalCount: Option<u32>, // Uncomment if needed
}

pub async fn get_all_devices(config: &Config, token: &Token) -> Result<Vec<Device>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let mut all_devices: Vec<Device> = Vec::new();
    let mut offset = 1; // Adjust based on API documentation (could be 0)
    let limit = 500;    // Set the limit as per API maximum

    loop {
        let devices_url = format!(
            "{}/dna/intent/api/v1/network-device?offset={}&limit={}",
            config.dnac_url, offset, limit
        );

        let mut resp = client
            .get(&devices_url)
            .header("X-Auth-Token", &token.value)
            .send()
            .await?;




        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            // Token might be invalid; re-authenticate
            eprintln!("Token expired or invalid. Re-authenticating...");

            // Re-authenticate
            let new_token = auth::authenticate(config).await?;

            // Retry the request with the new token
            resp = client
                .get(&devices_url)
                .header("X-Auth-Token", &new_token.value)
                .send()
                .await?;
        }

        if !resp.status().is_success() {
            return Err(anyhow!(
                "Failed to get devices: {}",
                resp.status()
            ));
        }

        let devices_resp: DevicesResponse = resp.json().await?;
        let devices = devices_resp.response;

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
