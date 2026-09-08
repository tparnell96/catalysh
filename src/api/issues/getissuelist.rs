// src/api/issues/getissuelist.rs

use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct IssueListResponse {
    pub version: Option<String>,
    pub totalCount: Option<String>,
    pub response: Option<Vec<Issue>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Issue {
    pub issueId: Option<String>,
    pub name: Option<String>,
    pub siteId: Option<String>,
    pub deviceId: Option<String>,
    pub deviceRole: Option<String>,
    pub aiDriven: Option<String>,
    pub clientMac: Option<String>,
    pub issue_occurence_count: Option<i32>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub category: Option<String>,
    pub last_occurence_time: Option<i64>,
}

pub async fn get_issue_list(
    config: &Config,
    token: &Token,
    search_params: &HashMap<String, String>,
) -> Result<IssueListResponse> {
    let client = http::build_client(config)?;

    let url = format!("{}/dna/intent/api/v1/issues", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&search_params)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("Failed to retrieve issue list: {}", resp.status()));
    }

    let issue_list_response = resp.json::<IssueListResponse>().await?;
    Ok(issue_list_response)
}
