pub mod config;
pub mod update; // Added this line

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AppCommands {
    /// App configuration commands
    Config {
        #[command(subcommand)]
        subcommand: config::AppConfigCommands,
    },
    /// Update the program to the latest release available (Program restart needed for changes to take effect)
    Update,
    // Additional app-related subcommands can be added here
}

