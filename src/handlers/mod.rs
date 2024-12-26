pub mod show;
pub mod config;
pub mod app;

use std::process::Command;

pub use show::handle_show_command;
pub use config::handle_config_command;
pub use app::handle_app_command;

pub fn clear_screen() -> std::io::Result<()> {
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
