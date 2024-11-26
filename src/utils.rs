use crate::api::devices::getdevicelist::AllDevices;
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
