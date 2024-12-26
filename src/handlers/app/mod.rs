pub mod config;
pub mod update;

use std::process::Command;
use log::error;
use crate::commands::app::AppCommands;

pub fn handle_app_command(subcommand: AppCommands) {
    match subcommand {
        AppCommands::Config { subcommand } => config::handle_app_config_command(subcommand),
        AppCommands::Update => update::handle_update_command(),
        AppCommands::Clear => {
            if let Err(e) = clear_screen() {
                error!("Failed to clear screen: {}", e);
            }
        },
    }
}

fn clear_screen() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()?;
    } else {
        // Unix-like systems (Linux, macOS)
        Command::new("clear")
            .status()?;
    }
    Ok(())
}

