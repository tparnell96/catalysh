#![allow(dead_code)]
// src/api/devices/compliance.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplianceRecord {
    pub device_uuid: Option<String>,
    pub compliance_type: Option<String>,
    pub status: Option<String>,
    pub last_sync_time: Option<i64>,
    pub additional_data_url: Option<String>,
    pub source_info_list: Option<serde_json::Value>,
    pub ack_status: Option<String>,
    pub ack_updated_by: Option<String>,
    pub ack_updated_time: Option<i64>,
    pub version: Option<String>,
    pub remediation_supported: Option<bool>,
    pub last_update_time: Option<i64>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ComplianceListResponse {
    pub response: Option<Vec<ComplianceRecord>>,
    pub version: Option<String>,
}

pub async fn get_compliance(
    config: &Config,
    token: &Token,
    compliance_type: Option<&str>,
) -> Result<ComplianceListResponse> {
    let client = http::build_client(config)?;
    let url = if let Some(ct) = compliance_type {
        format!(
            "{}/dna/intent/api/v1/compliance?complianceType={}",
            config.dnac_url, ct
        )
    } else {
        format!("{}/dna/intent/api/v1/compliance", config.dnac_url)
    };
    http::get_authenticated(&client, config, token, &url).await
}
