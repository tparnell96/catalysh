use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::http;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Site {
    pub id: Option<String>,
    pub name: Option<String>,
    pub site_name_hierarchy: Option<String>,
    pub parent_id: Option<String>,
    pub group_type_list: Option<Vec<String>>,
    pub group_hierarchy: Option<String>,
    pub additional_info: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct SitesResponse {
    response: Vec<Site>,
}

pub async fn get_all_sites(config: &Config, token: &Token) -> Result<Vec<Site>> {
    let client = http::build_client(config)?;

    let sites_url = format!("{}/dna/intent/api/v1/site", config.dnac_url);

    let resp = client
        .get(&sites_url)
        .header("X-Auth-Token", &token.value)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!("Failed to fetch sites: {}", resp.status()));
    }

    let sites_response: SitesResponse = resp.json().await?;
    Ok(sites_response.response)
}
