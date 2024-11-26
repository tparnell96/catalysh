use clap::{Parser, Subcommand};
use super::handlers::{handle_devices, handle_config, handle_update};

// Main command structure
#[derive(Debug, Parser)]
#[command(name = "catsh", about = "A REPL CLI for managing network devices")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// Commands structure with subcommands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage Devices in Catalyst Center
    Devices {
        #[command(subcommand)]
        subcommand: DeviceSubcommands,
    },
    /// Manage the configuration of catsh
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommands,
    },
    /// Update the program to the latest release available (Program restart needed for changes to take effect)
    Update,
    /// Exit the program
    Exit,
}

// Device-specific subcommands
#[derive(Debug, Subcommand)]
pub enum DeviceSubcommands {
    All,
    Details {
        #[arg(help = "Device ID or Name")]
        device_id: String,
    },
}

// Config-specific subcommands
#[derive(Debug, Subcommand)]
pub enum ConfigSubcommands {
    /// Remove the program config and prompt the user on the next command run for an endpoint, SSL setting, username, and password
    Reset,
}

// Functions to route commands to their respective handlers
pub fn route_command(command: Commands) {
    match command {
        Commands::Devices { subcommand } => handle_devices(subcommand),
        Commands::Config { subcommand } => handle_config(subcommand),
        Commands::Update => handle_update(),
        Commands::Exit => {
            println!("Exiting catsh...");
            std::process::exit(0);
        }
    }
}
