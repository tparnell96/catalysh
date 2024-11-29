// src/helpers/utils.rs

use crate::api::devices::devicedetailenrichment::DeviceDetails;
use crate::api::devices::getdevicelist::AllDevices;
use crate::api::clients::getclientdetail::ClientDetailResponse;
use chrono::Utc;
use prettytable::{row, Table};

pub fn current_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

pub fn print_devices(devices: Vec<AllDevices>) {
    let mut table = Table::new();
    table.add_row(row![
        "Hostname",
        "Mac Address",
        "Ethernet MAC Address",
        "IP Address",
        "Serial Number",
        "Associated WLC",
        "Software Version"
    ]);

    for device in devices {
        table.add_row(row![
            device.hostname.unwrap_or_else(|| "N/A".to_string()),
            device.mac_address.unwrap_or_else(|| "N/A".to_string()),
            device.ap_ethernet_mac_address.unwrap_or_else(|| "N/A".to_string()),
            device
                .management_ip_address
                .unwrap_or_else(|| "N/A".to_string()),
            device.serial_number.unwrap_or_else(|| "N/A".to_string()),
            device.associated_wlc_ip.unwrap_or_else(|| "N/A".to_string()),
            device.software_version.unwrap_or_else(|| "N/A".to_string()),
        ]);
    }

    table.printstd();
}

pub fn print_device_detail(device: AllDevices) {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);

    table.add_row(row![
        "Hostname",
        device.hostname.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Management IP Address",
        device
            .management_ip_address
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "MAC Address",
        device.mac_address.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Serial Number",
        device.serial_number.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Software Version",
        device.software_version.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Associated WLC IP",
        device.associated_wlc_ip.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Description",
        device.description.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Family",
        device.family.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Type",
        device.device_type.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Reachability Status",
        device
            .reachability_status
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    // Add more fields as needed

    table.printstd();
}

pub fn print_device_enrichment(device_details: DeviceDetails) {
    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);

    table.add_row(row![
        "Hostname",
        device_details.hostname.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Management IP Address",
        device_details
            .managementIpAddress
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "MAC Address",
        device_details
            .macAddress
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Serial Number",
        device_details
            .serialNumber
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Software Version",
        device_details
            .softwareVersion
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Associated WLC IP",
        device_details
            .associatedWlcIp
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Description",
        device_details
            .errorDescription
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Family",
        device_details.family.unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Type",
        device_details
            .type_field
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    table.add_row(row![
        "Reachability Status",
        device_details
            .reachabilityStatus
            .unwrap_or_else(|| "N/A".to_string())
    ]);
    // Add more fields as needed

    table.printstd();
}

pub fn print_client_detail(response: ClientDetailResponse) {
    if let Some(detail) = response.detail {
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);

        table.add_row(row![
            "ID",
            detail.id.unwrap_or_else(|| "N/A".to_string())
        ]);
        table.add_row(row![
            "Connection Status",
            detail.connectionStatus.unwrap_or_else(|| "N/A".to_string())
        ]);
        table.add_row(row![
            "Host Type",
            detail.hostType.unwrap_or_else(|| "N/A".to_string())
        ]);
        table.add_row(row![
            "User ID",
            detail.userId.unwrap_or_else(|| "N/A".to_string())
        ]);
        table.add_row(row![
            "Host Name",
            detail.hostName.unwrap_or_else(|| "N/A".to_string())
        ]);
        table.add_row(row![
            "MAC Address",
            detail.hostMac.unwrap_or_else(|| "N/A".to_string())
        ]);
        table.add_row(row![
            "IPv4 Address",
            detail.hostIpV4.unwrap_or_else(|| "N/A".to_string())
        ]);
        // Add more fields as needed

        table.printstd();
    } else {
        println!("No client details available.");
    }
}
