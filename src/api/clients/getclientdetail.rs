// src/api/clients/getclientdetail.rs

use crate::app::config::Config;
use crate::api::authentication::auth::Token;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ClientDetailResponse {
    pub detail: Option<ClientDetail>,
    pub connectionInfo: Option<ConnectionInfo>,
    pub topology: Option<Topology>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ClientDetail {
    pub id: Option<String>,
    pub connectionStatus: Option<String>,
    pub hostType: Option<String>,
    pub userId: Option<String>,
    pub hostName: Option<String>,
    pub hostMac: Option<String>,
    pub hostIpV4: Option<String>,
    // Add other fields as needed based on the schema
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ConnectionInfo {
    // Define fields as per the schema
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Topology {
    // Define fields as per the schema
}

pub async fn get_client_detail(
    config: &Config,
    token: &Token,
    mac_address: &str,
) -> Result<ClientDetailResponse> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}/dna/intent/api/v1/client-detail", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&[("macAddress", mac_address)])
        // .query(&[("timestamp", timestamp)]) // Include if timestamp is used
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve client details: {}",
            resp.status()
        ));
    }

    let client_detail_response = resp.json::<ClientDetailResponse>().await?;
    Ok(client_detail_response)
}
