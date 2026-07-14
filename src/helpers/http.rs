// src/helpers/http.rs
// Shared HTTP client utilities to avoid per-module client construction

use crate::api::authentication::auth::{self, Token};
use crate::app::config::Config;
use anyhow::{anyhow, Result};
use reqwest::Client;

/// Build a `reqwest::Client` respecting the SSL verification setting.
pub fn build_client(config: &Config) -> Result<Client> {
    Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()
        .map_err(|e| anyhow!("Failed to build HTTP client: {}", e))
}

/// Send an authenticated GET request, reauthenticating once on 401.
pub async fn get_authenticated<T>(
    client: &Client,
    config: &Config,
    token: &Token,
    url: &str,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut current_token = token.clone();

    loop {
        let resp = client
            .get(url)
            .header("X-Auth-Token", &current_token.value)
            .send()
            .await?;

        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            eprintln!("Token expired. Reauthenticating...");
            current_token = auth::authenticate(config).await?;
            continue;
        }

        if !resp.status().is_success() {
            return Err(anyhow!("Request to {} failed: {}", url, resp.status()));
        }

        return Ok(resp.json::<T>().await?);
    }
}
