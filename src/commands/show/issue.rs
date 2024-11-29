// src/commands/show/issue.rs

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Subcommand)]
pub enum IssueCommands {
    /// List issues based on search criteria
    List {
        /// Search option (e.g., deviceId, macAddress, priority, etc.)
        #[arg(value_enum)]
        search_option: Option<SearchOption>,
        /// Search input corresponding to the search option
        search_input: Option<String>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SearchOption {
    /// Start time to search from when looking for issues
    StartTime,
    /// End time used in conjunction with StartTime
    EndTime,
    /// SiteID gotten from a show site detail command
    SiteId,
    /// DeviceID gotten from a show device detail command
    DeviceId,
    /// MAC Address of a device or client
    MacAddress,
    /// One of these options - P1, P2, P3, P4
    Priority,
    /// Only pull issues are/aren't AI Driven - must be "Yes" or "No"
    AiDriven,
    /// Only pull issues with a specific status
    IssueStatus,
}
