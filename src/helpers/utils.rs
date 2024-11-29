// src/helpers/utils.rs

#[allow(unused_imports)]
use crate::api::clients::getclientdetail::{
    ClientDetailResponse, HealthScore, ConnectedDevice, Onboarding, ConnectionInfo,
    Topology, TopologyNode, TopologyLink,
};
use crate::api::devices::getdevicelist::AllDevices;
use crate::api::devices::devicedetailenrichment::DeviceDetails;
use crate::api::wireless::accesspointconfig::ApConfig;
#[allow(unused_imports)]
use crate::api::issues::getissuelist::{IssueListResponse, Issue};
use chrono::{DateTime, Utc};
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
        "Ethernet MAC Address",
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
            device.ap_ethernet_mac_address.unwrap_or_else(|| "N/A".to_string()),
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
        let datetime = DateTime::from_timestamp_millis(timestamp as i64)
            .unwrap_or_else(|| DateTime::from_timestamp(0, 0).expect("REASON"));
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
            let datetime = DateTime::from_timestamp_millis(timestamp as i64)
                .unwrap_or_else(|| DateTime::from_timestamp(0, 0).expect("REASON"));
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
                let datetime = DateTime::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_else(|| DateTime::from_timestamp(0, 0).expect("REASON"));
                add_field(
                    &mut table,
                    "Onboarding - Auth Done Time",
                    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
                );
            } else {
                add_field(&mut table, "Onboarding - Auth Done Time", None);
            }

            if let Some(timestamp) = onboarding.assocDoneTime {
                let datetime = DateTime::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_else(|| DateTime::from_timestamp(0, 0).expect("REASON"));
                add_field(
                    &mut table,
                    "Onboarding - Assoc Done Time",
                    Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
                );
            } else {
                add_field(&mut table, "Onboarding - Assoc Done Time", None);
            }

            if let Some(timestamp) = onboarding.dhcpDoneTime {
                let datetime = DateTime::from_timestamp_millis(timestamp as i64)
                    .unwrap_or_else(|| DateTime::from_timestamp(0, 0).expect("REASON"));
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
            let datetime = DateTime::from_timestamp_millis(timestamp as i64)
                .unwrap_or_else(|| DateTime::from_timestamp(0, 0).expect("REASON"));
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

pub fn print_issue_list(response: IssueListResponse) {
    if let Some(issues) = response.response {
        let mut table = Table::new();
        table.add_row(row![
            "Issue ID",
            "Name",
            "Device ID",
            "Device Role",
            "Client MAC",
            "Status",
            "Priority",
            "Category",
            "Last Occurrence Time"
        ]);

        for issue in issues {
            let last_occurrence = issue.last_occurence_time.map_or("N/A".to_string(), |timestamp| {
                DateTime::from_timestamp_millis(timestamp)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Invalid Timestamp".to_string())
            });

            table.add_row(row![
                issue.issueId.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.name.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.deviceId.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.deviceRole.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.clientMac.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.status.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.priority.clone().unwrap_or_else(|| "N/A".to_string()),
                issue.category.clone().unwrap_or_else(|| "N/A".to_string()),
                last_occurrence,
            ]);
        }

        table.printstd();
    } else {
        println!("No issues found.");
    }
}

// Function to print AP configuration
pub fn print_ap_config(ap_config: ApConfig) {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);

    add_field(&mut table, "Instance UUID", ap_config.instanceUuid.map(|v| v.to_string()));
    add_field(&mut table, "Instance ID", ap_config.instanceId.map(|v| v.to_string()));
    add_field(&mut table, "Display Name", ap_config.displayName);
    add_field(&mut table, "Instance Tenant ID", ap_config.instanceTenantId);
    add_field(
        &mut table,
        "Ordered List OE Index",
        ap_config._orderedListOEIndex.map(|v| v.to_string()),
    );
    add_field(
        &mut table,
        "Creation Order Index",
        ap_config._creationOrderIndex.map(|v| v.to_string()),
    );
    add_field(
        &mut table,
        "Is Being Changed",
        ap_config._isBeingChanged.map(|v| v.to_string()),
    );
    add_field(&mut table, "Deploy Pending", ap_config.deployPending);
    add_field(&mut table, "Instance Version", ap_config.instanceVersion.map(|v| v.to_string()));
    add_field(&mut table, "Admin Status", ap_config.adminStatus);
    add_field(&mut table, "AP Height", ap_config.apHeight.map(|v| v.to_string()));
    add_field(&mut table, "AP Mode", ap_config.apMode);
    add_field(&mut table, "AP Name", ap_config.apName);
    add_field(&mut table, "Ethernet MAC", ap_config.ethMac);
    add_field(&mut table, "Failover Priority", ap_config.failoverPriority);
    add_field(
        &mut table,
        "LED Brightness Level",
        ap_config.ledBrightnessLevel.map(|v| v.to_string()),
    );
    add_field(&mut table, "LED Status", ap_config.ledStatus);
    add_field(&mut table, "Location", ap_config.location);
    add_field(&mut table, "MAC Address", ap_config.macAddress);
    add_field(&mut table, "Primary Controller Name", ap_config.primaryControllerName);
    add_field(&mut table, "Primary IP Address", ap_config.primaryIpAddress);
    add_field(&mut table, "Secondary Controller Name", ap_config.secondaryControllerName);
    add_field(&mut table, "Secondary IP Address", ap_config.secondaryIpAddress);
    add_field(&mut table, "Tertiary Controller Name", ap_config.tertiaryControllerName);
    add_field(&mut table, "Tertiary IP Address", ap_config.tertiaryIpAddress);

    // Internal Key
    if let Some(internal_key) = ap_config.internalKey {
        add_field(&mut table, "Internal Key - Type", internal_key.type_field);
        add_field(&mut table, "Internal Key - ID", internal_key.id.map(|v| v.to_string()));
        add_field(&mut table, "Internal Key - Long Type", internal_key.longType);
        add_field(&mut table, "Internal Key - URL", internal_key.url);
    }

    // Display the table
    table.printstd();

    // Mesh DTOs - Since the schema shows as an array of empty objects, we can skip or handle as needed.

    // Radio DTOs
    if let Some(radio_dtos) = ap_config.radioDTOs {
        for (i, radio) in radio_dtos.iter().enumerate() {
            println!("\nRadio DTO [{}]:", i + 1);
            let mut radio_table = Table::new();
            radio_table.add_row(row!["Field", "Value"]);

            add_field(&mut radio_table, "Display Name", radio.displayName.clone());
            add_field(&mut radio_table, "Instance ID", radio.instanceId.map(|v| v.to_string()));
            add_field(
                &mut radio_table,
                "Ordered List OE Index",
                radio._orderedListOEIndex.map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Creation Order Index",
                radio._creationOrderIndex.map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Is Being Changed",
                radio._isBeingChanged.map(|v| v.to_string()),
            );
            add_field(&mut radio_table, "Deploy Pending", radio.deployPending.clone());
            add_field(
                &mut radio_table,
                "Instance Version",
                radio.instanceVersion.map(|v| v.to_string()),
            );
            add_field(&mut radio_table, "Admin Status", radio.adminStatus.clone());
            add_field(
                &mut radio_table,
                "Antenna Angle",
                radio.antennaAngle.map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Antenna Elevation Angle",
                radio.antennaElevAngle.map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Antenna Gain",
                radio.antennaGain.map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Antenna Pattern Name",
                radio.antennaPatternName.clone(),
            );
            add_field(
                &mut radio_table,
                "Channel Assignment Mode",
                radio.channelAssignmentMode.clone(),
            );
            add_field(
                &mut radio_table,
                "Channel Number",
                radio.channelNumber.map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Channel Width",
                radio.channelWidth.clone(),
            );
            add_field(&mut radio_table, "Clean Air SI", radio.cleanAirSI.clone());
            add_field(&mut radio_table, "Interface Type", radio.ifType.map(|v| v.to_string()));
            add_field(
                &mut radio_table,
                "Interface Type Value",
                radio.ifTypeValue.clone(),
            );
            add_field(&mut radio_table, "MAC Address", radio.macAddress.clone());
            add_field(
                &mut radio_table,
                "Power Assignment Mode",
                radio.powerAssignmentMode.clone(),
            );
            add_field(
                &mut radio_table,
                "Power Level",
                radio.powerlevel.map(|v| v.to_string()),
            );
            // radioBand and radioRoleAssignment are Option<serde_json::Value>; handle accordingly
            add_field(
                &mut radio_table,
                "Radio Band",
                radio.radioBand.as_ref().map(|v| v.to_string()),
            );
            add_field(
                &mut radio_table,
                "Radio Role Assignment",
                radio.radioRoleAssignment.as_ref().map(|v| v.to_string()),
            );
            add_field(&mut radio_table, "Slot ID", radio.slotId.map(|v| v.to_string()));

            // Internal Key for RadioDTO
            if let Some(radio_internal_key) = &radio.internalKey {
                add_field(
                    &mut radio_table,
                    "Internal Key - Type",
                    radio_internal_key.type_field.clone(),
                );
                add_field(
                    &mut radio_table,
                    "Internal Key - ID",
                    radio_internal_key.id.map(|v| v.to_string()),
                );
                add_field(
                    &mut radio_table,
                    "Internal Key - Long Type",
                    radio_internal_key.longType.clone(),
                );
                add_field(
                    &mut radio_table,
                    "Internal Key - URL",
                    radio_internal_key.url.clone(),
                );
            }

            // Display the radio table
            radio_table.printstd();
        }
    }
}

