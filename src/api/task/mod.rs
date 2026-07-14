#![allow(dead_code)]
// src/api/task/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Option<String>,
    pub data: Option<String>,
    pub service_type: Option<String>,
    pub is_error: Option<bool>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub username: Option<String>,
    pub progress: Option<String>,
    pub error_code: Option<String>,
    pub failure_reason: Option<String>,
    pub instance_tenant_id: Option<String>,
    pub root_id: Option<String>,
    pub parent_id: Option<String>,
    pub version: Option<i64>,
    pub last_update: Option<i64>,
    pub operation_id_list: Option<serde_json::Value>,
    pub additional_status_url: Option<String>,
    pub result_location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub response: Option<Vec<Task>>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDetailResponse {
    pub response: Option<Task>,
    pub version: Option<String>,
}

pub async fn list_tasks(
    config: &Config,
    token: &Token,
    offset: u32,
    limit: u32,
) -> Result<TaskListResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/task?offset={}&limit={}",
        config.dnac_url, offset, limit
    );
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_task(config: &Config, token: &Token, task_id: &str) -> Result<TaskDetailResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/task/{}", config.dnac_url, task_id);
    http::get_authenticated(&client, config, token, &url).await
}
