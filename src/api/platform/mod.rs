// src/api/platform/mod.rs
use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnacRelease {
    pub version: Option<String>,
    pub name: Option<String>,
    pub installed_version: Option<String>,
    pub system_version: Option<String>,
    pub packages: Option<Vec<DnacPackage>>,
    pub core_packages_install_status: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct DnacReleaseResponse {
    pub response: Option<DnacRelease>,
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnacPackage {
    pub name: Option<String>,
    pub version: Option<String>,
    pub install_mode: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DnacPackagesResponse {
    pub response: Option<Vec<DnacPackage>>,
    pub version: Option<String>,
}

pub async fn get_release(config: &Config, token: &Token) -> Result<DnacReleaseResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/dnac-release", config.dnac_url);
    http::get_authenticated(&client, config, token, &url).await
}

pub async fn get_packages(config: &Config, token: &Token) -> Result<DnacPackagesResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/dnac-packages", config.dnac_url);
    http::get_authenticated(&client, config, token, &url).await
}
