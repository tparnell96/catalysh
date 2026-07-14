// src/commands/show/eox.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum EoxCommands {
    /// Show EoX summary counts
    Summary,
    /// List devices with EoX alerts
    Devices {
        /// Maximum number of devices to return
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Show EoX detail for a specific device
    Device {
        /// Device UUID
        device_id: String,
    },
}
