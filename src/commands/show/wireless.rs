// src/commands/show/wireless.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum WirelessCommands {
    /// Show SSIDs for a site
    Ssid {
        /// Site ID
        #[arg(long)]
        site_id: String,
    },
}
