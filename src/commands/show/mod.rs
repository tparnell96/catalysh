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
    /// Show site information
    Site {
        #[command(subcommand)]
        subcommand: site::SiteCommands,
    },
    /// Show topology information
    Topology {
        #[command(subcommand)]
        subcommand: topology::TopologyCommands,
    },
    /// Show health scores
    Health {
        #[command(subcommand)]
        subcommand: health::HealthCommands,
    },
    /// Show task information
    Task {
        #[command(subcommand)]
        subcommand: task::TaskCommands,
    },
    /// Show End-of-Life / End-of-Support status
    Eox {
        #[command(subcommand)]
        subcommand: eox::EoxCommands,
    },
    /// Show security advisories
    Advisory {
        #[command(subcommand)]
        subcommand: advisory::AdvisoryCommands,
    },
    /// Show path trace / flow analysis
    Path {
        #[command(subcommand)]
        subcommand: path::PathCommands,
    },
    /// Show discovery jobs
    Discovery {
        #[command(subcommand)]
        subcommand: discovery::DiscoveryCommands,
    },
    /// Show platform release and package info
    Platform {
        #[command(subcommand)]
        subcommand: platform::PlatformCommands,
    },
    /// Show network settings
    NetworkSettings {
        #[command(subcommand)]
        subcommand: networksettings::NetworkSettingsCommands,
    },
    /// Show tags
    Tag {
        #[command(subcommand)]
        subcommand: tag::TagCommands,
    },
    /// Show wireless settings
    Wireless {
        #[command(subcommand)]
        subcommand: wireless::WirelessCommands,
    },
}
