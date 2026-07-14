pub mod advisory;
pub mod ap;
pub mod client;
pub mod device;
pub mod discovery;
pub mod eox;
pub mod health;
pub mod issue;
pub mod networksettings;
pub mod path;
pub mod platform;
pub mod site;
pub mod tag;
pub mod task;
pub mod topology;
pub mod wireless;
use crate::commands::show::ShowCommands;

pub fn handle_show_command(subcommand: ShowCommands) {
    match subcommand {
        ShowCommands::Device { subcommand } => device::handle_device_command(subcommand),
        ShowCommands::Client { subcommand } => client::handle_client_command(subcommand),
        ShowCommands::Issue { subcommand } => issue::handle_issue_command(subcommand),
        ShowCommands::Ap { subcommand } => ap::handle_ap_command(subcommand),
        ShowCommands::Site { subcommand } => site::handle_site_command(subcommand),
        ShowCommands::Topology { subcommand } => topology::handle_topology_command(subcommand),
        ShowCommands::Health { subcommand } => health::handle_health_command(subcommand),
        ShowCommands::Task { subcommand } => task::handle_task_command(subcommand),
        ShowCommands::Eox { subcommand } => eox::handle_eox_command(subcommand),
        ShowCommands::Advisory { subcommand } => advisory::handle_advisory_command(subcommand),
        ShowCommands::Path { subcommand } => path::handle_path_command(subcommand),
        ShowCommands::Discovery { subcommand } => discovery::handle_discovery_command(subcommand),
        ShowCommands::Platform { subcommand } => platform::handle_platform_command(subcommand),
        ShowCommands::NetworkSettings { subcommand } => {
            networksettings::handle_networksettings_command(subcommand)
        }
        ShowCommands::Tag { subcommand } => tag::handle_tag_command(subcommand),
        ShowCommands::Wireless { subcommand } => wireless::handle_wireless_command(subcommand),
    }
}
