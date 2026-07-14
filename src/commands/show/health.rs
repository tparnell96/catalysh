// src/commands/show/health.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum HealthCommands {
    /// Show network health
    Network {
        /// Optional site ID filter
        #[arg(long)]
        site_id: Option<String>,
    },
    /// Show client health
    Client {
        /// Optional site ID filter
        #[arg(long)]
        site_id: Option<String>,
    },
}
