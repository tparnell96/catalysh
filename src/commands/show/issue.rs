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
    StartTime,
    EndTime,
    SiteId,
    DeviceId,
    MacAddress,
    Priority,
    AiDriven,
    IssueStatus,
}
