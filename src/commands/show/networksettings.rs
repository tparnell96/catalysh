// src/commands/show/networksettings.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum NetworkSettingsCommands {
    /// Show global network settings (DNS, NTP, domain)
    Global,
}
