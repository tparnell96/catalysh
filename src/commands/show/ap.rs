// src/commands/show/ap.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ApCommands {
    /// Show AP configuration by MAC address
    Config {
        /// MAC address of the AP
        mac_address: String,
    },
    /// Show all RF profiles
    RfProfile,
}
