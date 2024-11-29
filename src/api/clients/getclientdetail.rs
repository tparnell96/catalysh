// src/api/clients/getclientdetail.rs

use crate::app::config::Config;
use crate::api::authentication::auth::Token;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ClientDetailResponse {
    pub detail: Option<ClientDetail>,
    pub connectionInfo: Option<ConnectionInfo>,
    pub topology: Option<Topology>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ClientDetail {
    pub id: Option<String>,
    pub connectionStatus: Option<String>,
    pub hostType: Option<String>,
    pub userId: Option<String>,
    pub hostName: Option<String>,
    pub hostOs: Option<String>,
    pub hostVersion: Option<String>,
    pub subType: Option<String>,
    pub lastUpdated: Option<u64>, // Changed from Option<String> to Option<u64>
    pub healthScore: Option<Vec<HealthScore>>,
    pub hostMac: Option<String>,
    pub hostIpV4: Option<String>,
    pub hostIpV6: Option<Vec<String>>,
    pub authType: Option<String>,
    pub vlanId: Option<i32>,
    pub vnid: Option<i32>,
    pub ssid: Option<String>,
    pub frequency: Option<String>,
    pub channel: Option<String>,
    pub apGroup: Option<String>,
    pub location: Option<String>,
    pub clientConnection: Option<String>,
    pub connectedDevice: Option<Vec<ConnectedDevice>>,
    pub issueCount: Option<i32>,
    pub rssi: Option<String>,
    pub avgRssi: Option<String>,
    pub snr: Option<String>,
    pub avgSnr: Option<String>,
    pub dataRate: Option<String>,
    pub txBytes: Option<String>,
    pub rxBytes: Option<String>,
    pub onboarding: Option<Onboarding>,
    pub clientType: Option<String>,
    pub onboardingTime: Option<u64>, // Changed from Option<String> to Option<u64>
    pub port: Option<String>,
    pub iosCapable: Option<bool>,
    pub tracked: Option<String>,
    pub duid: Option<String>,
    pub identifier: Option<String>,
    pub firmwareVersion: Option<String>,
    pub deviceVendor: Option<String>,
    pub deviceForm: Option<String>,
    pub salesCode: Option<String>,
    pub countryCode: Option<String>,
    pub l3VirtualNetwork: Option<String>,
    pub l2VirtualNetwork: Option<String>,
    pub upnId: Option<String>,
    pub upnName: Option<String>,
    pub sgt: Option<String>,
    pub rssiThreshold: Option<String>,
    pub rssiIsInclude: Option<String>,
    pub snrThreshold: Option<String>,
    pub snrIsInclude: Option<String>,
    pub dnsResponse: Option<String>,
    pub dnsRequest: Option<String>,
    pub usage: Option<f64>,
    pub linkSpeed: Option<f64>,
    pub linkThreshold: Option<String>,
    pub remoteEndDuplexMode: Option<String>,
    pub txLinkError: Option<f64>,
    pub rxLinkError: Option<f64>,
    pub txRate: Option<f64>,
    pub rxRate: Option<f64>,
    pub rxRetryPct: Option<String>,
    pub versionTime: Option<i64>,
    pub dot11Protocol: Option<String>,
    pub slotId: Option<i32>,
    pub dot11ProtocolCapability: Option<String>,
    pub privateMac: Option<bool>,
    pub dhcpServerIp: Option<String>,
    pub aaaServerIp: Option<String>,
    pub aaaServerTransaction: Option<i32>,
    pub aaaServerFailedTransaction: Option<i32>,
    pub aaaServerSuccessTransaction: Option<i32>,
    pub aaaServerLatency: Option<f64>,
    pub aaaServerMABLatency: Option<f64>,
    pub aaaServerEAPLatency: Option<f64>,
    pub dhcpServerTransaction: Option<i32>,
    pub dhcpServerFailedTransaction: Option<i32>,
    pub dhcpServerSuccessTransaction: Option<i32>,
    pub dhcpServerLatency: Option<f64>,
    pub dhcpServerDOLatency: Option<f64>,
    pub dhcpServerRALatency: Option<f64>,
    pub maxRoamingDuration: Option<String>,
    pub upnOwner: Option<String>,
    pub connectedUpn: Option<String>,
    pub connectedUpnOwner: Option<String>,
    pub connectedUpnId: Option<String>,
    pub isGuestUPNEndpoint: Option<bool>,
    pub wlcName: Option<String>,
    pub wlcUuid: Option<String>,
    pub sessionDuration: Option<String>,
    pub intelCapable: Option<bool>,
    pub hwModel: Option<String>,
    pub powerType: Option<String>,
    pub modelName: Option<String>,
    pub bridgeVMMode: Option<String>,
    pub dhcpNakIp: Option<String>,
    pub dhcpDeclineIp: Option<String>,
    pub portDescription: Option<String>,
    pub latencyVoice: Option<f64>,
    pub latencyVideo: Option<f64>,
    pub latencyBg: Option<f64>,
    pub latencyBe: Option<f64>,
    pub trustScore: Option<String>,
    pub trustDetails: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct HealthScore {
    pub healthType: Option<String>,
    pub reason: Option<String>,
    pub score: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ConnectedDevice {
    #[serde(rename = "type")]
    pub device_type: Option<String>,
    pub name: Option<String>,
    pub mac: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "ip address")]
    pub ip_address: Option<String>,
    pub mgmtIp: Option<String>,
    pub band: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Onboarding {
    pub averageRunDuration: Option<String>,
    pub maxRunDuration: Option<String>,
    pub averageAssocDuration: Option<String>,
    pub maxAssocDuration: Option<String>,
    pub averageAuthDuration: Option<String>,
    pub maxAuthDuration: Option<String>,
    pub averageDhcpDuration: Option<String>,
    pub maxDhcpDuration: Option<String>,
    pub aaaServerIp: Option<String>,
    pub dhcpServerIp: Option<String>,
    pub authDoneTime: Option<u64>,   // Changed from Option<String> to Option<u64>
    pub assocDoneTime: Option<u64>,  // Changed from Option<String> to Option<u64>
    pub dhcpDoneTime: Option<u64>,   // Changed from Option<String> to Option<u64>
    pub assocRootcauseList: Option<Vec<String>>,
    pub aaaRootcauseList: Option<Vec<String>>,
    pub dhcpRootcauseList: Option<Vec<String>>,
    pub otherRootcauseList: Option<Vec<String>>,
    pub latestRootCauseList: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ConnectionInfo {
    pub hostType: Option<String>,
    pub nwDeviceName: Option<String>,
    pub nwDeviceMac: Option<String>,
    pub protocol: Option<String>,
    pub band: Option<String>,
    pub spatialStream: Option<String>,
    pub channel: Option<String>,
    pub channelWidth: Option<String>,
    pub wmm: Option<String>,
    pub uapsd: Option<String>,
    pub timestamp: Option<u64>, // Changed from Option<String> to Option<u64>
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Topology {
    pub nodes: Option<Vec<TopologyNode>>,
    pub links: Option<Vec<TopologyLink>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TopologyNode {
    pub role: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub description: Option<String>,
    pub deviceType: Option<String>,
    pub platformId: Option<String>,
    pub family: Option<String>,
    pub ip: Option<String>,
    pub softwareVersion: Option<String>,
    pub userId: Option<String>,
    pub nodeType: Option<String>,
    pub radioFrequency: Option<String>,
    pub clients: Option<f64>,
    pub count: Option<i32>,
    pub healthScore: Option<f64>,
    pub level: Option<f64>,
    pub fabricGroup: Option<String>,
    pub connectedDevice: Option<String>,
    pub fabricRole: Option<Vec<String>>,
    pub stackType: Option<String>,
    pub ipv6: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct TopologyLink {
    pub source: Option<String>,
    pub linkStatus: Option<String>,
    pub label: Option<Vec<String>>,
    pub target: Option<String>,
    pub id: Option<String>,
    pub portUtilization: Option<f64>,
}

pub async fn get_client_detail(
    config: &Config,
    token: &Token,
    mac_address: &str,
) -> Result<ClientDetailResponse> {
    let client = Client::builder()
        .danger_accept_invalid_certs(!config.verify_ssl)
        .build()?;

    let url = format!("{}/dna/intent/api/v1/client-detail", config.dnac_url);

    let resp = client
        .get(&url)
        .header("X-Auth-Token", &token.value)
        .query(&[("macAddress", mac_address)])
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Failed to retrieve client details: {}",
            resp.status()
        ));
    }

    let client_detail_response = resp.json::<ClientDetailResponse>().await?;
    Ok(client_detail_response)
}
