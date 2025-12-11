// src/commands/show/site.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SiteCommands {
    /// List all sites
    List,
}
