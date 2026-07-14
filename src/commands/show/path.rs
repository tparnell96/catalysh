// src/commands/show/path.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum PathCommands {
    /// List all path trace flows
    List,
    /// Start a path trace
    Trace {
        /// Source IP address
        #[arg(long)]
        source_ip: String,
        /// Destination IP address
        #[arg(long)]
        dest_ip: String,
        /// Source port (optional)
        #[arg(long)]
        source_port: Option<String>,
        /// Destination port (optional)
        #[arg(long)]
        dest_port: Option<String>,
        /// Protocol (optional)
        #[arg(long)]
        protocol: Option<String>,
    },
    /// Get path trace detail by flow analysis ID
    Get {
        /// Flow analysis ID
        flow_id: String,
    },
}
