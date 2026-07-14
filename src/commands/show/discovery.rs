// src/commands/show/discovery.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DiscoveryCommands {
    /// List discoveries
    List {
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Get discovery detail by ID
    Get {
        /// Discovery ID
        id: String,
    },
}
