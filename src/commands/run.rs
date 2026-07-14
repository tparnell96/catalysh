// src/commands/run.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum RunCommands {
    /// CLI command runner
    Command {
        #[command(subcommand)]
        subcommand: CommandRunnerCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum CommandRunnerCommands {
    /// Execute read-only CLI commands on devices
    Exec {
        /// Comma-separated device UUIDs
        #[arg(long)]
        devices: String,
        /// Commands to execute (space-separated)
        #[arg(long, num_args = 1..)]
        commands: Vec<String>,
    },
    /// List allowed read-only CLI commands
    LegitReads,
}
