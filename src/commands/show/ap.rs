// src/commands/show/ap.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ApCommands {
    /// Show AP configuration — accepts hostname, management IP, or any MAC address format
    Config {
        /// Hostname, management IP, ethernet MAC, or radio MAC of the AP
        selector: String,
    },
    /// Show all RF profiles
    RfProfile,
}
