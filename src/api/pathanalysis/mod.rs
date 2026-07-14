#![allow(dead_code)]
// src/api/pathanalysis/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisSummary {
    pub id: Option<String>,
    pub source_i_p: Option<String>,
    pub dest_i_p: Option<String>,
    pub source_port: Option<String>,
    pub dest_port: Option<String>,
    pub protocol: Option<String>,
    pub status: Option<String>,
    pub create_time: Option<i64>,
    pub last_update_time: Option<i64>,
    pub periodic_refresh: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FlowAnalysisListResponse {
    pub response: Option<Vec<FlowAnalysisSummary>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisIdResult {
    pub flow_analysis_id: Option<String>,
    pub url: Option<String>,
    pub task_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FlowAnalysisTraceResponse {
    pub response: Option<FlowAnalysisIdResult>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkElementInfo {
    pub name: Option<String>,
    pub id: Option<String>,
    pub ip: Option<String>,
    pub link_information_source: Option<String>,
    pub r#type: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisDetail {
    pub id: Option<String>,
    pub source_i_p: Option<String>,
    pub dest_i_p: Option<String>,
    pub source_port: Option<String>,
    pub dest_port: Option<String>,
    pub protocol: Option<String>,
    pub status: Option<String>,
    pub network_elements_info: Option<Vec<NetworkElementInfo>>,
    pub last_update_time: Option<i64>,
    pub create_time: Option<i64>,
    pub periodic_refresh: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FlowAnalysisDetailResponse {
    pub response: Option<FlowAnalysisDetail>,
    pub version: Option<String>,
}

pub async fn list_flow_analyses(
    config: &Config,
    token: &Token,
) -> Result<FlowAnalysisListResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/flow-analysis", config.dnac_url);
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn start_path_trace(
    config: &Config,
    token: &Token,
    source_ip: &str,
    dest_ip: &str,
    source_port: Option<&str>,
    dest_port: Option<&str>,
    protocol: Option<&str>,
) -> Result<FlowAnalysisTraceResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/flow-analysis", config.dnac_url);
    let mut body = serde_json::json!({
        "sourceIP": source_ip,
        "destIP": dest_ip
    });
    if let Some(sp) = source_port {
        body["sourcePort"] = serde_json::Value::String(sp.to_string());
    }
    if let Some(dp) = dest_port {
        body["destPort"] = serde_json::Value::String(dp.to_string());
    }
    if let Some(proto) = protocol {
        body["protocol"] = serde_json::Value::String(proto.to_string());
    }

    let resp = client
        .post(&url)
        .header("X-Auth-Token", &token.value)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("POST flow-analysis failed: {}", resp.status()));
    }

    Ok(resp.json::<FlowAnalysisTraceResponse>().await?)
}

pub async fn get_flow_analysis(
    config: &Config,
    token: &Token,
    flow_id: &str,
) -> Result<FlowAnalysisDetailResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/flow-analysis/{}",
        config.dnac_url, flow_id
    );
    http::get_authenticated(&client, config, token, &url).await
}
