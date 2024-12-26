use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AppConfigCommands {
    /// Reset app configuration
    Reset,
    /// Show current configuration
    Show,
}

