use crate::api::authentication::auth::Token;
use crate::app::config::Config;
use crate::helpers::{http, utils};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientHealthScoreCategory {
    pub score_category: Option<String>,
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientHealthScore {
    pub score_category: Option<ClientHealthScoreCategory>,
    pub score_value: Option<serde_json::Value>,
    pub client_count: Option<serde_json::Value>,
    pub client_unique_count: Option<serde_json::Value>,
    pub starttime: Option<i64>,
    pub endtime: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientHealthSite {
    pub site_id: Option<String>,
    pub score_detail: Vec<ClientHealthScore>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientHealthResponse {
    pub response: Vec<ClientHealthSite>,
}

pub async fn get_client_health(
    config: &Config,
    token: &Token,
    timestamp_ms: Option<i64>,
) -> Result<ClientHealthResponse> {
    let client = http::build_client(config)?;
    let url = format!("{}/dna/intent/api/v1/client-health", config.dnac_url);
    let query = [(
        "timestamp",
        timestamp_ms
            .unwrap_or_else(|| utils::current_timestamp() as i64)
            .to_string(),
    )];

    http::get_authenticated_with_query(&client, config, token, &url, &query).await
}
