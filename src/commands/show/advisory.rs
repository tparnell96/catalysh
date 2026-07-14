// src/commands/show/advisory.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AdvisoryCommands {
    /// List all security advisories
    List,
    /// Show advisories for a specific device
    Device {
        /// Device UUID
        device_id: String,
    },
    /// Show advisory aggregate counts
    Aggregate,
}
