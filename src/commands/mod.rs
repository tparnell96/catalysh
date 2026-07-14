pub mod app;
pub mod run;
pub mod show;
pub mod tui;
pub mod workflow;

use crate::handlers::{
    clear_screen, handle_app_command, handle_run_command, handle_show_command,
    handle_tui_command, handle_workflow_command,
};
use crate::helpers::output::OutputFormat;
use clap::{Parser, Subcommand};
use log::error;

#[derive(Debug, Parser)]
#[command(
    name = "catalysh",
    about = "A command line interface for Cisco Catalyst Center"
)]
pub struct Cli {
    /// Output format
    #[arg(
        long,
        short = 'o',
        global = true,
        value_enum,
        default_value = "table",
        help = "Output format: table (default) or json"
    )]
    pub output: OutputFormat,

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
    /// Catalyst Center workflow commands (PnP, diagnostic, device replacement)
    Workflow {
        #[command(subcommand)]
        subcommand: workflow::WorkflowCommands,
    },
    /// App-specific commands
    App {
        #[command(subcommand)]
        subcommand: app::AppCommands,
    },
    /// Interactive TUI dashboards
    Tui {
        #[command(subcommand)]
        subcommand: tui::TuiCommands,
    },
    /// Clear the screen
    Clear,
    /// Exit the program
    Exit,
}

pub fn route_command(cli: Cli) {
    // Apply the output format globally before dispatching
    crate::helpers::output::set(cli.output);

    match cli.command {
        Commands::Show { subcommand } => handle_show_command(subcommand),
        Commands::Run { subcommand } => handle_run_command(subcommand),
        Commands::Workflow { subcommand } => handle_workflow_command(subcommand),
        Commands::App { subcommand } => handle_app_command(subcommand),
        Commands::Tui { subcommand } => handle_tui_command(subcommand),
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
