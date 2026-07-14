use crate::commands::tui::TuiCommands;
use crate::tui::health;
use log::error;

pub fn handle_tui_command(subcommand: TuiCommands) {
    match subcommand {
        TuiCommands::Health => {
            if let Err(e) = health::launch_health_tui() {
                error!("TUI error: {}", e);
            }
        }
    }
}
