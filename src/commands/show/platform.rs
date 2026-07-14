// src/commands/show/platform.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum PlatformCommands {
    /// Show platform release information
    Release,
    /// List installed packages
    Packages,
}
