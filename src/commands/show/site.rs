// src/commands/show/site.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SiteCommands {
    /// List all sites
    List,
    /// Show site health scores
    Health {
        /// Optional site type filter (area, building, floor)
        #[arg(long)]
        site_type: Option<String>,
    },
}
