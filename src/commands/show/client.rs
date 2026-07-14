// src/commands/show/client.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ClientCommands {
    /// Show client details by MAC address
    Detail {
        /// MAC address of the client
        mac_address: String,
    },
    /// Show client enrichment by network user ID or MAC address
    Enrichment {
        /// The entity type (network_user_id or mac_address)
        #[arg(value_parser = ["network_user_id", "mac_address"])]
        entity_type: String,
        /// The value of the entity (user ID or MAC address)
        entity_value: String,
        /// Optional issue category
        #[arg(long)]
        issue_category: Option<String>,
    },
    /// List clients with optional filters
    List {
        /// Filter by MAC address
        #[arg(long)]
        mac: Option<String>,
        /// Filter by IPv4 address
        #[arg(long)]
        ipv4: Option<String>,
        /// Filter by SSID
        #[arg(long)]
        ssid: Option<String>,
        /// Filter by type (wired or wireless)
        #[arg(long, value_parser = ["wired", "wireless"])]
        client_type: Option<String>,
        /// Filter by site ID
        #[arg(long)]
        site_id: Option<String>,
        /// Max results
        #[arg(long, default_value = "100")]
        limit: u32,
    },
    /// Show which APs/locations a user has connected from recently
    Proximity {
        /// Username to look up
        username: String,
        /// Number of days to look back
        #[arg(long, default_value = "14")]
        days: u32,
    },
}
