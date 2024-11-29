// src/commands/show/client.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ClientCommands {
    /// Show client details by MAC address
    Detail {
        /// MAC address of the client
        mac_address: String,
    },
}
