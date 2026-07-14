// src/api/workflows/pnp.rs
// PnP (Plug-and-Play) workflow endpoints.
// GET  /dna/intent/api/v1/onboarding/pnp-workflow
// GET  /dna/intent/api/v1/onboarding/pnp-workflow/{id}
// GET  /dna/intent/api/v1/onboarding/pnp-workflow/count

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PnpWorkflowTask {
    pub name: Option<String>,
    pub task_seq_no: Option<i64>,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub state: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub time_taken: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnpWorkflow {
    #[serde(rename = "_id")]
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub workflow_type: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "useState")]
    pub use_state: Option<String>,
    #[serde(rename = "addedOn")]
    pub added_on: Option<i64>,
    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,
    pub tasks: Option<Vec<PnpWorkflowTask>>,
    #[serde(rename = "addToInventory")]
    pub add_to_inventory: Option<bool>,
    #[serde(rename = "tenantId")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PnpWorkflowCountResponse {
    response: Option<i64>,
}

pub async fn list_pnp_workflows(
    config: &Config,
    token: &Token,
    limit: u32,
    offset: u32,
) -> Result<Vec<PnpWorkflow>> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/onboarding/pnp-workflow?limit={}&offset={}",
        config.dnac_url, limit, offset
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_pnp_workflow_by_id(
    config: &Config,
    token: &Token,
    id: &str,
) -> Result<PnpWorkflow> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/onboarding/pnp-workflow/{}",
        config.dnac_url, id
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_pnp_workflow_count(config: &Config, token: &Token) -> Result<i64> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/onboarding/pnp-workflow/count",
        config.dnac_url
    );
    let resp: PnpWorkflowCountResponse =
        http::get_authenticated(&client, config, token, &url).await?;
    Ok(resp.response.unwrap_or(0))
}
