use crate::app::config::Config;
use crate::api::authentication::auth;
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse {
    #[serde(rename = "response")]
    pub response: Vec<RFProfile>,
    #[serde(rename = "version")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioTypeProperties {
    #[serde(rename = "parentProfile")]
    pub parent_profile: Option<String>,
    #[serde(rename = "radioChannels")]
    pub radio_channels: Option<String>,
    #[serde(rename = "dataRates")]
    pub data_rates: Option<String>,
    #[serde(rename = "mandatoryDataRates")]
    pub mandatory_data_rates: Option<String>,
    #[serde(rename = "powerThresholdV1")]
    pub power_threshold_v1: Option<f64>,
    #[serde(rename = "rxSopThreshold")]
    pub rx_sop_threshold: Option<String>,
    #[serde(rename = "minPowerLevel")]
    pub min_power_level: Option<i32>,
    #[serde(rename = "maxPowerLevel")]
    pub max_power_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RFProfile {
    pub name: Option<String>,
    #[serde(rename = "defaultRfProfile")]
    pub default_rf_profile: Option<bool>,
    #[serde(rename = "enableRadioTypeA")]
    pub enable_radio_type_a: Option<bool>,
    #[serde(rename = "enableRadioTypeB")]
    pub enable_radio_type_b: Option<bool>,
    #[serde(rename = "enableRadioTypeC")]  
    pub enable_radio_type_c: Option<bool>,
    #[serde(rename = "channelWidth")]
    pub channel_width: Option<String>,
    #[serde(rename = "enableCustom")]
    pub enable_custom: Option<bool>,
    #[serde(rename = "enableBrownField")]
    pub enable_brown_field: Option<bool>,
    #[serde(rename = "radioTypeAProperties")]
    pub radio_type_a_properties: Option<RadioTypeProperties>,
    #[serde(rename = "radioTypeBProperties")]
    pub radio_type_b_properties: Option<RadioTypeProperties>,
    #[serde(rename = "radioTypeCProperties")]
    pub radio_type_c_properties: Option<RadioTypeProperties>,
}

const RF_PROFILE_ENDPOINT: &str = "/dna/intent/api/v1/wireless/rf-profile";

pub async fn get_all_rf_profiles(config: &Config, token: &auth::Token) -> Result<Vec<RFProfile>> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}{}", config.dnac_url, RF_PROFILE_ENDPOINT);
    debug!("Requesting RF profiles from URL: {}", url);

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("X-Auth-Token", &token.value)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    let response_text = response.text().await?;
    debug!("Raw API Response: {}", response_text);

    // Parse into Value first for debugging
    let response_value: Value = serde_json::from_str(&response_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse response as JSON: {}", e))?;
    debug!("Response structure: {:#?}", response_value);

    let api_response: APIResponse = serde_json::from_str(&response_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse RF profiles response: {}", e))?;
    debug!("Parsed {} RF profiles", api_response.response.len());

    Ok(api_response.response)
}
