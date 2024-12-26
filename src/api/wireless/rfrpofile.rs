use crate::api::Client;
use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RadioProperties {
    #[serde(rename = "parentProfile")]
    parent_profile: String,
    #[serde(rename = "radioChannels")]
    radio_channels: String,
    #[serde(rename = "dataRates")]
    data_rates: String,
    #[serde(rename = "mandatoryDataRates")]
    mandatory_data_rates: String,
    #[serde(rename = "powerThresholdV1")]
    power_threshold_v1: f64,
    #[serde(rename = "rxSopThreshold")]
    rx_sop_threshold: String,
    #[serde(rename = "minPowerLevel")]
    min_power_level: i32,
    #[serde(rename = "maxPowerLevel")]
    max_power_level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RFProfile {
    pub name: String,
    #[serde(rename = "defaultRfProfile")]
    pub default_rf_profile: bool,
    #[serde(rename = "enableRadioTypeA")]
    pub enable_radio_type_a: bool,
    #[serde(rename = "enableRadioTypeB")]
    pub enable_radio_type_b: bool,
    #[serde(rename = "channelWidth")]
    pub channel_width: String,
    #[serde(rename = "enableCustom")]
    pub enable_custom: bool,
    #[serde(rename = "enableBrownField")]
    pub enable_brown_field: bool,
    #[serde(rename = "radioTypeAProperties")]
    pub radio_type_a_properties: RadioProperties,
    #[serde(rename = "radioTypeBProperties")]
    pub radio_type_b_properties: RadioProperties,
    #[serde(rename = "radioTypeCProperties")]
    pub radio_type_c_properties: RadioProperties,
    #[serde(rename = "enableRadioTypeC")]
    pub enable_radio_type_c: bool,
}

impl Client {
    /// Fetches all RF profiles from the DNA Center.
    ///
    /// # Errors
    /// 
    /// Returns an error if the API request fails or if the response cannot be parsed.
    pub async fn get_rf_profiles(&self) -> Result<Vec<RFProfile>, Error> {
        let profiles: Vec<RFProfile> = self
            .get("/dna/intent/api/v1/wireless/rf-profile")
            .await?;
        Ok(profiles)
    }
}

