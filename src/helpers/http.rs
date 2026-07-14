// src/helpers/http.rs
// Shared HTTP client utilities to avoid per-module client construction

use crate::api::authentication::auth::{self, Token};
use crate::app::config::Config;
use anyhow::{anyhow, Result};
use reqwest::{Certificate, Client};

/// Build a `reqwest::Client` respecting the SSL verification setting and any
/// custom CA certificates installed via `app config install-cert`.
pub fn build_client(config: &Config) -> Result<Client> {
    let mut builder = Client::builder().danger_accept_invalid_certs(!config.verify_ssl);

    // Load custom CA certs from ~/.config/catalysh/certs/
    if config.verify_ssl {
        let certs_dir = crate::app::config::get_certs_dir();
        if certs_dir.exists() {
            for entry in std::fs::read_dir(&certs_dir)
                .map_err(|e| anyhow!("Failed to read certs dir: {}", e))?
            {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let pem = std::fs::read(&path)
                        .map_err(|e| anyhow!("Failed to read cert '{}': {}", path.display(), e))?;
                    match Certificate::from_pem(&pem) {
                        Ok(cert) => {
                            builder = builder.add_root_certificate(cert);
                        }
                        Err(_) => {
                            // Try DER format as fallback
                            if let Ok(cert) = Certificate::from_der(&pem) {
                                builder = builder.add_root_certificate(cert);
                            } else {
                                log::warn!(
                                    "Skipping unreadable certificate file: {}",
                                    path.display()
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    builder
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
