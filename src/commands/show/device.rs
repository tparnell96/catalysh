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
    /// Show CDP/LLDP neighbors for any device
    Neighbors {
        /// Hostname, management IP address, or MAC address
        selector: String,
    },
    /// Show total device count
    Count,
    /// Show device health scores
    Health {
        /// Optional device role filter (e.g. ACCESS, CORE, DISTRIBUTION, BORDER ROUTER)
        #[arg(long)]
        device_role: Option<String>,
    },
    /// Show device compliance status
    Compliance {
        /// Optional compliance type filter
        #[arg(long)]
        compliance_type: Option<String>,
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
    /// List devices filtered by WLC IP address
    Wlc {
        /// Optional partial WLC ip to filter by
        partial_wlc: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceDetailFilter {
    /// Show device detail by any identifier (hostname, IP, or MAC)
    By {
        /// Hostname, management IP address, or MAC address
        selector: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceEnrichmentFilter {
    /// Enrichment by any identifier (hostname, IP, or MAC) — entity type is auto-detected
    By {
        /// Hostname, management IP address, or MAC address
        selector: String,
    },
}
