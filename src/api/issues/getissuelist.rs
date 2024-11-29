// src/api/issues/getissuelist.rs

use crate::app::config::Config;
use crate::api::authentication::auth::Token;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct IssueListResponse {
    pub version: Option<String>,
    pub totalCount: Option<String>,
    pub response: Option<Vec<Issue>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
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
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}/dna/intent/api/v1/issues", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&search_params)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve issue list: {}",
            resp.status()
        ));
    }

    let issue_list_response = resp.json::<IssueListResponse>().await?;
    Ok(issue_list_response)
}
