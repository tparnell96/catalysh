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
}
