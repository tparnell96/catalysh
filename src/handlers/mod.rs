pub mod app;
pub mod run;
pub mod show;
pub mod workflow;

use std::process::Command;

pub use app::handle_app_command;
pub use run::handle_run_command;
pub use show::handle_show_command;
pub use workflow::handle_workflow_command;

pub fn clear_screen() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status()?;
    } else {
        Command::new("clear").status()?;
    }
    Ok(())
}
