pub mod device;
pub mod client;
pub mod issue;
pub mod ap;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ShowCommands {
    /// Show device information
    Device {
        #[command(subcommand)]
        subcommand: device::DeviceCommands,
    },
    /// Show client information
    Client {
        #[command(subcommand)]
        subcommand: client::ClientCommands,
    },
    /// Show issues in Catalyst Center
    Issue {
        #[command(subcommand)]
        subcommand: issue::IssueCommands,
    },
    /// Show Access Point information
    Ap {
        #[command(subcommand)]
        subcommand: ap::ApCommands,
    },
}

