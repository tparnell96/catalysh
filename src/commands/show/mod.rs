pub mod device;
pub mod client;

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
}

