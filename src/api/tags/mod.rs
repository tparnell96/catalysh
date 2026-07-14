#![allow(dead_code)]
// src/api/tags/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub system_tag: Option<bool>,
    pub dynamic_rules: Option<serde_json::Value>,
    pub instance_tenant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TagListResponse {
    pub response: Option<Vec<Tag>>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagMembersResponse {
    pub response: Option<serde_json::Value>,
    pub version: Option<String>,
}

pub async fn list_tags(
    config: &Config,
    token: &Token,
    name: Option<&str>,
) -> Result<TagListResponse> {
    let client = http::build_client(config)?;
    let url = if let Some(n) = name {
        format!("{}/dna/intent/api/v1/tag?name={}", config.dnac_url, n)
    } else {
        format!("{}/dna/intent/api/v1/tag", config.dnac_url)
    };
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_tag_members(
    config: &Config,
    token: &Token,
    tag_id: &str,
) -> Result<TagMembersResponse> {
    let client = http::build_client(config)?;
    let url = format!(
        "{}/dna/intent/api/v1/tag/{}/member",
        config.dnac_url, tag_id
    );
    http::get_authenticated(&client, config, token, &url).await
}
