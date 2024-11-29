pub mod device;
pub mod client;
pub mod issue;
pub mod ap;
use crate::commands::show::ShowCommands;

pub fn handle_show_command(subcommand: ShowCommands) {
    match subcommand {
        ShowCommands::Device { subcommand } => device::handle_device_command(subcommand),
    
        ShowCommands::Client { subcommand } => client::handle_client_command(subcommand),

        ShowCommands::Issue { subcommand } => issue::handle_issue_command(subcommand),

        ShowCommands::Ap { subcommand } => ap::handle_ap_command(subcommand),
    }
}

