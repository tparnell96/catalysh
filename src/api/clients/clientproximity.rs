// src/api/clients/clientproximity.rs

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProximityResponse {
    pub execution_id: Option<String>,
    pub execution_status_url: Option<String>,
    pub message: Option<String>,
}

pub async fn get_client_proximity(
    config: &Config,
    token: &Token,
    username: &str,
    number_days: Option<u32>,
    time_resolution: Option<u32>,
) -> Result<ProximityResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/client-proximity", config.dnac_url);

    let days = number_days.unwrap_or(14);
    let resolution = time_resolution.unwrap_or(15);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&[
            ("username", username.to_string()),
            ("number_days", days.to_string()),
            ("time_resolution", resolution.to_string()),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve client proximity: {}",
            resp.status()
        ));
    }

    let proximity_resp = resp.json::<ProximityResponse>().await?;
    Ok(proximity_resp)
}
