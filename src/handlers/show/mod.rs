pub mod device;

use crate::commands::show::ShowCommands;

pub fn handle_show_command(subcommand: ShowCommands) {
    match subcommand {
        ShowCommands::Device { subcommand } => device::handle_device_command(subcommand),
    }
}

