// src/helpers/utils.rs

use crate::api::clients::getclientdetail::{
    ClientDetailResponse, HealthScore, ConnectedDevice, Onboarding, ConnectionInfo,
    Topology, TopologyNode, TopologyLink,
};
use crate::api::devices::getdevicelist::AllDevices;
use crate::api::devices::devicedetailenrichment::DeviceDetails;
use chrono::{NaiveDateTime, Utc};
use prettytable::{row, Table};

pub fn current_timestamp() -> u64 {
    Utc::now().timestamp_millis() as u64
}

// Function to print a list of devices
pub fn print_devices(devices: Vec<AllDevices>) {
    let mut table = Table::new();
    table.add_row(row![
        "Hostname",
        "Management IP",
        "Serial Number",
        "MAC Address",
        "Platform ID",
        "Software Version",
        "Role"
    ]);

    for device in devices {
        table.add_row(row![
            device.hostname.unwrap_or_else(|| "N/A".to_string()),
            device.management_ip_address.unwrap_or_else(|| "N/A".to_string()),
            device.serial_number.unwrap_or_else(|| "N/A".to_string()),
            device.mac_address.unwrap_or_else(|| "N/A".to_string()),
            device.platform_id.unwrap_or_else(|| "N/A".to_string()),
            device.software_version.unwrap_or_else(|| "N/A".to_string()),
            device.role.unwrap_or_else(|| "N/A".to_string()),
        ]);
    }

    table.printstd();
}

// Function to print detailed information about a device
pub fn print_device_detail(device: AllDevices) {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);

    add_field(&mut table, "Hostname", device.hostname);
    add_field(
        &mut table,
        "Management IP",
        device.management_ip_address,
    );
    add_field(&mut table, "Serial Number", device.serial_number);
    add_field(&mut table, "MAC Address", device.mac_address);
    add_field(&mut table, "Platform ID", device.platform_id);
    add_field(&mut table, "Software Version", device.software_version);
    add_field(&mut table, "Role", device.role);
    add_field(&mut table, "Reachability Status", device.reachability_status);
    add_field(&mut table, "Uptime", device.up_time);
    add_field(&mut table, "Last Updated", device.last_update_time.map(|timestamp| {
        let datetime = NaiveDateTime::from_timestamp_millis(timestamp as i64)
            .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }));
    // Add more fields as necessary

    table.printstd();
}

// Function to print enriched device details
pub fn print_device_enrichment(device_details: DeviceDetails) {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);

    add_field(&mut table, "Hostname", device_details.hostname);
    add_field(
        &mut table,
        "Management IP",
        device_details.managementIpAddress,
    );
    add_field(&mut table, "Serial Number", device_details.serialNumber);
    add_field(&mut table, "MAC Address", device_details.macAddress);
    add_field(
        &mut table,
        "Platform ID",
        device_details.platformId,
    );
    add_field(
        &mut table,
        "Software Version",
        device_details.softwareVersion,
    );
    add_field(
        &mut table,
        "Reachability Status",
        device_details.reachabilityStatus,
    );
    add_field(
        &mut table,
        "Error Code",
        device_details.errorCode.map(|v| v.to_string()),
    );
    add_field(
        &mut table,
        "Error Description",
        device_details.errorDescription,
    );
    // Add more fields as necessary

    table.printstd();
}

// Function to print client detail with all fields
pub fn print_client_detail(response: ClientDetailResponse) {
    if let Some(detail) = response.detail {
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);

        add_field(&mut table, "ID", detail.id);
        add_field(&mut table, "Connection Status", detail.connectionStatus);
        add_field(&mut table, "Host Type", detail.hostType);
        add_field(&mut table, "User ID", detail.userId);
        add_field(&mut table, "Host Name", detail.hostName);
        add_field(&mut table, "Host OS", detail.hostOs);
        add_field(&mut table, "Host Version", detail.hostVersion);
        add_field(&mut table, "Sub Type", detail.subType);

        // lastUpdated as timestamp
        if let Some(timestamp) = detail.lastUpdated {
            let datetime = NaiveDateTime::from_timestamp_millis(timestamp as i64)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
            add_field(
                &mut table,
                "Last Updated",
                Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
            );
        } else {
            add_field(&mut table, "Last Updated", None);
        }

        // Health Score
        if let Some(health_scores) = detail.healthScore {
            for (i, hs) in health_scores.iter().enumerate() {
                let prefix = format!("Health Score [{}]", i + 1);
                add_field(&mut table, &format!("{} - Health Type", prefix), hs.healthType.clone());
                add_field(&mut table, &format!("{} - Reason", prefix), hs.reason.clone());
                add_field(
                    &mut table,
                    &format!("{} - Score", prefix),
                    hs.score.map(|s| s.to_string()),
                );
            }
        }

        add_field(&mut table, "Host MAC", detail.hostMac);
        add_field(&mut table, "Host IPv4", detail.hostIpV4);
        add_field(
            &mut table,
            "Host IPv6",
            detail.hostIpV6.map(|ips| ips.join(", ")),
        );
        add_field(&mut table, "Auth Type", detail.authType);
        add_field(
            &mut table,
            "VLAN ID",
            detail.vlanId.map(|v| v.to_string()),
        );
        add_field(
            &mut table,
            "VNID",
            detail.vnid.map(|v| v.to_string()),
        );
        add_field(&mut table, "SSID", detail.ssid);
        add_field(&mut table, "Frequency", detail.frequency);
        add_field(&mut table, "Channel", detail.channel);
        add_field(&mut table, "AP Group", detail.apGroup);
        add_field(&mut table, "Location", detail.location);
        add_field(&mut table, "Client Connection", detail.clientConnection);

        // Connected Devices
        if let Some(connected_devices) = detail.connectedDevice {
            for (i, cd) in connected_devices.iter().enumerate() {
                let prefix = format!("Connected Device [{}]", i + 1);
                add_field(&mut table, &format!("{} - Type", prefix), cd.device_type.clone());
                add_field(&mut table, &format!("{} - Name", prefix), cd.name.clone());
                add_field(&mut table, &format!("{} - MAC", prefix), cd.mac.clone());
                add_field(&mut table, &format!("{} - ID", prefix), cd.id.clone());
                add_field(&mut table, &format!("{} - IP Address", prefix), cd.ip_address.clone());
                add_field(&mut table, &format!("{} - Mgmt IP", prefix), cd.mgmtIp.clone());
                add_field(&mut table, &format!("{} - Band", prefix), cd.band.clone());
                add_field(&mut table, &format!("{} - Mode", prefix), cd.mode.clone());
            }
        }

        add_field(
            &mut table,
            "Issue Count",
            detail.issueCount.map(|v| v.to_string()),
        );
        add_field(&mut table, "RSSI", detail.rssi);
        add_field(&mut table, "Average RSSI", detail.avgRssi);
        add_field(&mut table, "SNR", detail.snr);
        add_field(&mut table, "Average SNR", detail.avgSnr);
        add_field(&mut table, "Data Rate", detail.dataRate);
        add_field(&mut table, "TX Bytes", detail.txBytes);
        add_field(&mut table, "RX Bytes", detail.rxBytes);

        // Onboarding
        if let Some(onboarding) = detail.onboarding {
            // Timestamps
            if let Some(timestamp) = onboarding.authDoneTime {
                let datetime = NaiveDateTime::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
                add_field(
                    &mut table,
                    "Onboarding - Auth Done Time",
                    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
                );
            } else {
                add_field(&mut table, "Onboarding - Auth Done Time", None);
            }

            if let Some(timestamp) = onboarding.assocDoneTime {
                let datetime = NaiveDateTime::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
                add_field(
                    &mut table,
                    "Onboarding - Assoc Done Time",
                    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
                );
            } else {
                add_field(&mut table, "Onboarding - Assoc Done Time", None);
            }

            if let Some(timestamp) = onboarding.dhcpDoneTime {
                let datetime = NaiveDateTime::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
                add_field(
                    &mut table,
                    "Onboarding - DHCP Done Time",
                    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
                );
            } else {
                add_field(&mut table, "Onboarding - DHCP Done Time", None);
            }

            // Other onboarding fields
            add_field(
                &mut table,
                "Onboarding - Average Run Duration",
                onboarding.averageRunDuration,
            );
            add_field(
                &mut table,
                "Onboarding - Max Run Duration",
                onboarding.maxRunDuration,
            );
            // Add more onboarding fields as necessary

            // Root cause lists
            if let Some(assoc_rc_list) = onboarding.assocRootcauseList {
                add_field(
                    &mut table,
                    "Onboarding - Assoc Rootcause List",
                    Some(assoc_rc_list.join(", ")),
                );
            }
            if let Some(aaa_rc_list) = onboarding.aaaRootcauseList {
                add_field(
                    &mut table,
                    "Onboarding - AAA Rootcause List",
                    Some(aaa_rc_list.join(", ")),
                );
            }
            if let Some(dhcp_rc_list) = onboarding.dhcpRootcauseList {
                add_field(
                    &mut table,
                    "Onboarding - DHCP Rootcause List",
                    Some(dhcp_rc_list.join(", ")),
                );
            }
            if let Some(other_rc_list) = onboarding.otherRootcauseList {
                add_field(
                    &mut table,
                    "Onboarding - Other Rootcause List",
                    Some(other_rc_list.join(", ")),
                );
            }
            if let Some(latest_rc_list) = onboarding.latestRootCauseList {
                add_field(
                    &mut table,
                    "Onboarding - Latest Rootcause List",
                    Some(latest_rc_list.join(", ")),
                );
            }
        }

        // Continue adding all other fields as needed

        table.printstd();
    } else {
        println!("No client details available.");
    }

    // Optionally, print ConnectionInfo and Topology
    if let Some(connection_info) = response.connectionInfo {
        println!("\nConnection Info:");
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);

        add_field(&mut table, "Host Type", connection_info.hostType);
        add_field(&mut table, "Network Device Name", connection_info.nwDeviceName);
        add_field(&mut table, "Network Device MAC", connection_info.nwDeviceMac);
        add_field(&mut table, "Protocol", connection_info.protocol);
        add_field(&mut table, "Band", connection_info.band);
        add_field(&mut table, "Spatial Stream", connection_info.spatialStream);
        add_field(&mut table, "Channel", connection_info.channel);
        add_field(&mut table, "Channel Width", connection_info.channelWidth);
        add_field(&mut table, "WMM", connection_info.wmm);
        add_field(&mut table, "UAPSD", connection_info.uapsd);

        // Timestamp
        if let Some(timestamp) = connection_info.timestamp {
            let datetime = NaiveDateTime::from_timestamp_millis(timestamp as i64)
                .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
            add_field(
                &mut table,
                "Timestamp",
                Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
            );
        } else {
            add_field(&mut table, "Timestamp", None);
        }

        table.printstd();
    }

    if let Some(topology) = response.topology {
        println!("\nTopology Information:");
        // You can choose to display topology data as needed
        if let Some(nodes) = topology.nodes {
            for node in nodes {
                let mut table = Table::new();
                table.add_row(row!["Node Field", "Value"]);
                add_field(&mut table, "Role", node.role);
                add_field(&mut table, "Name", node.name);
                add_field(&mut table, "ID", node.id);
                add_field(&mut table, "Description", node.description);
                add_field(&mut table, "Device Type", node.deviceType);
                add_field(&mut table, "Platform ID", node.platformId);
                add_field(&mut table, "Family", node.family);
                add_field(&mut table, "IP", node.ip);
                add_field(&mut table, "Software Version", node.softwareVersion);
                add_field(&mut table, "User ID", node.userId);
                add_field(&mut table, "Node Type", node.nodeType);
                add_field(&mut table, "Radio Frequency", node.radioFrequency);
                add_field(
                    &mut table,
                    "Clients",
                    node.clients.map(|v| v.to_string()),
                );
                add_field(
                    &mut table,
                    "Count",
                    node.count.map(|v| v.to_string()),
                );
                add_field(
                    &mut table,
                    "Health Score",
                    node.healthScore.map(|v| v.to_string()),
                );
                add_field(
                    &mut table,
                    "Level",
                    node.level.map(|v| v.to_string()),
                );
                add_field(&mut table, "Fabric Group", node.fabricGroup);
                add_field(&mut table, "Connected Device", node.connectedDevice);
                if let Some(fabric_roles) = node.fabricRole {
                    add_field(
                        &mut table,
                        "Fabric Roles",
                        Some(fabric_roles.join(", ")),
                    );
                }
                if let Some(ipv6_list) = node.ipv6 {
                    add_field(&mut table, "IPv6", Some(ipv6_list.join(", ")));
                }

                table.printstd();
            }
        }

        // Similarly, you can display links if needed
    }
}

// Helper function to add a field to the table
fn add_field(table: &mut Table, field_name: &str, value: Option<String>) {
    table.add_row(row![
        field_name,
        value.unwrap_or_else(|| "N/A".to_string())
    ]);
}
