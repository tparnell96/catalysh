pub mod device;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ShowCommands {
    /// Show devices
    Device {
        #[command(subcommand)]
        subcommand: device::DeviceCommands,
    },
}

