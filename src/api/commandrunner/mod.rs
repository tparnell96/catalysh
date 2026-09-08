#![allow(dead_code)]
// src/api/commandrunner/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRunnerTaskResponse {
    pub task_id: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandRunnerResponse {
    pub response: Option<CommandRunnerTaskResponse>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LegitReadsResponse {
    pub response: Option<Vec<String>>,
    pub version: Option<String>,
}

pub async fn exec_commands(
    config: &Config,
    token: &Token,
    commands: Vec<String>,
    device_uuids: Vec<String>,
) -> Result<CommandRunnerResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/network-device-poller/cli/read-request",
        config.dnac_url
    );
    let body = serde_json::json!({
        "commands": commands,
        "deviceUuids": device_uuids
    });
    let resp = client
        .post(&url)
        .header("X-Auth-Token", &token.value)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("POST to {} failed: {}", url, resp.status()));
    }

    Ok(resp.json::<CommandRunnerResponse>().await?)
}

pub async fn get_legit_reads(config: &Config, token: &Token) -> Result<LegitReadsResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/network-device-poller/cli/legit-reads",
        config.dnac_url
    );
    http::get_authenticated(&client, config, token, &url).await
}
