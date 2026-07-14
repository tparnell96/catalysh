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
        /// Timestamp in milliseconds since epoch
        #[arg(long)]
        timestamp: Option<i64>,
    },
}
