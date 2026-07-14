// src/api/workflows/diagnostic.rs
// Diagnostic validation workflow endpoints.
// GET  /dna/intent/api/v1/diagnosticValidationWorkflows
// GET  /dna/intent/api/v1/diagnosticValidationWorkflows/{id}
// GET  /dna/intent/api/v1/diagnosticValidationWorkflows/count
// POST /dna/intent/api/v1/diagnosticValidationWorkflows  (submit/run)

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticWorkflow {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub run_status: Option<String>,
    pub submit_time: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub validation_status: Option<String>,
    pub validation_set_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiagnosticListResponse {
    response: Option<Vec<DiagnosticWorkflow>>,
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiagnosticDetailResponse {
    response: Option<DiagnosticWorkflow>,
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiagnosticCountResponse {
    response: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SubmitResponse {
    response: Option<SubmitResult>,
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResult {
    pub id: Option<String>,
    pub url: Option<String>,
}

/// List diagnostic validation workflows, optionally filtered by run status
/// (e.g. "PENDING", "IN_PROGRESS", "SUCCESS", "FAILED").
pub async fn list_diagnostic_workflows(
    config: &Config,
    token: &Token,
    run_status: Option<&str>,
    limit: u32,
    offset: u32,
) -> Result<Vec<DiagnosticWorkflow>> {
    let client = http::build_client(config)?;
    let mut url = format!(
        "{}/dna/intent/api/v1/diagnosticValidationWorkflows?limit={}&offset={}",
        config.dnac_url, limit, offset
    );
    if let Some(s) = run_status {
        url.push_str(&format!("&runStatus={}", s));
    }
    let resp: DiagnosticListResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response.unwrap_or_default())
}

pub async fn get_diagnostic_workflow(
    config: &Config,
    token: &Token,
    id: &str,
) -> Result<DiagnosticWorkflow> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/diagnosticValidationWorkflows/{}",
        config.dnac_url, id
    );
    let resp: DiagnosticDetailResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    resp.response
        .ok_or_else(|| anyhow!("No workflow detail returned for id '{}'", id))
}

pub async fn get_diagnostic_workflow_count(config: &Config, token: &Token) -> Result<i64> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/diagnosticValidationWorkflows/count",
        config.dnac_url
    );
    let resp: DiagnosticCountResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response.unwrap_or(0))
}

/// Submit a new diagnostic validation workflow run.
pub async fn submit_diagnostic_workflow(
    config: &Config,
    token: &Token,
    name: &str,
    description: Option<&str>,
    validation_set_ids: Vec<String>,
) -> Result<SubmitResult> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/diagnosticValidationWorkflows",
        config.dnac_url
    );
    let body = serde_json::json!({
        "name": name,
        "description": description.unwrap_or(""),
        "validationSetIds": validation_set_ids,
    });
    let resp = client
        .post(&url)
        .header("X-Auth-Token", &token.value)
        .json(&body)
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(anyhow!("Submit workflow failed: {}", resp.status()));
    }
    let result: SubmitResponse = resp.json().await?;
    result
        .response
        .ok_or_else(|| anyhow!("No response body from workflow submit"))
}
