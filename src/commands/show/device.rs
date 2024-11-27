// src/commands/show/device.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DeviceCommands {
    /// List devices
    List {
        #[command(subcommand)]
        filter: DeviceListFilter,
    },
    /// Show device details
    Detail {
        #[command(subcommand)]
        filter: DeviceDetailFilter,
    },
    /// Show device enrichment detail
    Enrichment {
        #[command(subcommand)]
        filter: DeviceEnrichmentFilter,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceListFilter {
    /// List all devices
    All,
    /// List devices filtered by hostname
    Hostname {
        /// Optional partial hostname to filter by
        partial_hostname: Option<String>,
    },
    /// List devices filtered by IP address
    Ip {
        /// Optional partial IP address to filter by
        partial_ip: Option<String>,
    },
    /// List devices filtered by WLC
    Wlc {
        /// Optional partial WLC name to filter by
        partial_wlc: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceDetailFilter {
    /// Show device detail by hostname
    Hostname {
        /// The hostname of the device
        hostname: String,
    },
    /// Show device detail by MAC address
    Mac {
        /// The MAC address of the device
        mac_address: String,
    },
    /// Show device detail by IP address
    Ip {
        /// The IP address of the device
        ip_address: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceEnrichmentFilter {
    /// Enrichment by MAC address
    Mac {
        /// The MAC address of the device
        mac_address: String,
    },
    /// Enrichment by IP address
    Ip {
        /// The IP address of the device
        ip_address: String,
    },
}
