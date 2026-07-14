pub mod app;
pub mod config;
pub mod run;
pub mod show;

use crate::handlers::{
    clear_screen, handle_app_command, handle_config_command, handle_run_command,
    handle_show_command,
};
use clap::{Parser, Subcommand};
use log::error;

#[derive(Debug, Parser)]
#[command(
    name = "catalysh",
    about = "A command line interface for Cisco Catalyst Center"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show commands
    Show {
        #[command(subcommand)]
        subcommand: show::ShowCommands,
    },
    /// Run commands on devices
    Run {
        #[command(subcommand)]
        subcommand: run::RunCommands,
    },
    /// Start configuration sub-REPL
    Config,
    /// App-specific commands
    App {
        #[command(subcommand)]
        subcommand: app::AppCommands,
    },
    /// Clear the screen
    Clear,
    /// Exit the program
    Exit,
}

pub fn route_command(command: Commands) {
    match command {
        Commands::Show { subcommand } => handle_show_command(subcommand),
        Commands::Run { subcommand } => handle_run_command(subcommand),
        Commands::Config => handle_config_command(),
        Commands::App { subcommand } => handle_app_command(subcommand),
        Commands::Clear => {
            if let Err(e) = clear_screen() {
                error!("Failed to clear screen: {}", e);
            }
        }
        Commands::Exit => {
            println!("Exiting catalysh...");
            std::process::exit(0);
        }
    }
}
