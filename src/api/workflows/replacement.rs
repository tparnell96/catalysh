// src/api/workflows/replacement.rs
// Device replacement workflow endpoints.
// GET  /dna/intent/api/v1/device-replacement  (list/status)
// POST /dna/intent/api/v1/device-replacement/workflow  (deploy)

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceReplacementStatus {
    pub id: Option<String>,
    pub faulty_device_name: Option<String>,
    pub faulty_device_serial_number: Option<String>,
    pub faulty_device_platform: Option<String>,
    pub faulty_device_id: Option<String>,
    pub replacement_device_serial_number: Option<String>,
    pub replacement_device_platform: Option<String>,
    pub replacement_status: Option<String>,
    pub creation_time: Option<i64>,
    pub replacement_time: Option<i64>,
    pub family: Option<String>,
    pub neighbor_device_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReplacementListResponse {
    response: Option<Vec<DeviceReplacementStatus>>,
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeployResponse {
    response: Option<serde_json::Value>,
    version: Option<String>,
}

pub async fn list_replacement_workflows(
    config: &Config,
    token: &Token,
) -> Result<Vec<DeviceReplacementStatus>> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/device-replacement",
        config.dnac_url
    );
    let resp: ReplacementListResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response.unwrap_or_default())
}

/// Deploy a device replacement workflow.
pub async fn deploy_replacement_workflow(
    config: &Config,
    token: &Token,
    faulty_serial: &str,
    replacement_serial: &str,
) -> Result<String> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/device-replacement/workflow",
        config.dnac_url
    );
    let body = serde_json::json!({
        "faultyDeviceSerialNumber": faulty_serial,
        "replacementDeviceSerialNumber": replacement_serial,
    });
    let resp = client
        .post(&url)
        .header("X-Auth-Token", &token.value)
        .json(&body)
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(anyhow!("Deploy replacement failed: {}", resp.status()));
    }
    let result: DeployResponse = resp.json().await?;
    Ok(result
        .response
        .and_then(|v| v.get("taskId").and_then(|t| t.as_str()).map(String::from))
        .unwrap_or_else(|| "submitted (no task ID returned)".to_string()))
}
