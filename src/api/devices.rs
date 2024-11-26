use crate::api::auth::{self, Token};
use crate::config::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
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
