pub mod app;
pub mod config;
pub mod run;
pub mod show;

use std::process::Command;

pub use app::handle_app_command;
pub use config::handle_config_command;
pub use run::handle_run_command;
pub use show::handle_show_command;

pub fn clear_screen() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()?;
    } else {
        // Unix-like systems (Linux, macOS)
        Command::new("clear").status()?;
    }
    Ok(())
}
