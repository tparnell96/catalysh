use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DeviceCommands {
    /// Show all devices
    All,
    /// Show details for a specific device
    Details {
        #[arg(help = "Device ID or Name")]
        device_id: String,
    },
}

