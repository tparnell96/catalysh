// src/commands/show/tag.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TagCommands {
    /// List tags
    List {
        /// Optional tag name filter
        #[arg(long)]
        name: Option<String>,
    },
    /// Show members of a tag
    Members {
        /// Tag ID
        tag_id: String,
    },
}
