// src/commands/workflow/mod.rs

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum WorkflowCommands {
    /// AP provisioning and factory-reset workflows
    Ap {
        #[command(subcommand)]
        subcommand: ApWorkflowCommands,
    },
    /// PnP (Plug-and-Play) onboarding workflows
    Pnp {
        #[command(subcommand)]
        subcommand: PnpCommands,
    },
    /// Diagnostic validation workflows
    Diagnostic {
        #[command(subcommand)]
        subcommand: DiagnosticCommands,
    },
    /// Device replacement workflows
    Replacement {
        #[command(subcommand)]
        subcommand: ReplacementCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum ApWorkflowCommands {
    /// Provision an AP to a site
    Provision {
        /// AP identifier: hostname, ethernet MAC, or IP
        #[arg(long)]
        ap: String,
        /// Full site hierarchy (e.g. "Global/Building1/Floor2")
        #[arg(long)]
        site: String,
        /// RF profile name (e.g. "HIGH")
        #[arg(long, default_value = "HIGH")]
        rf_profile: String,
    },
    /// Get AP provision status for a wireless controller
    Status {
        /// WLC selector: hostname, IP, or MAC
        #[arg(long)]
        controller: String,
    },
    /// Factory reset one or more APs
    FactoryReset {
        /// Comma-separated AP selectors (hostname, IP, or ethernet MAC)
        #[arg(long)]
        aps: String,
        /// Keep static IP configuration
        #[arg(long, default_value = "false")]
        keep_static_ip: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum PnpCommands {
    /// List all PnP workflows
    List {
        /// Max results to return (default: 50)
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Result offset for pagination (default: 0)
        #[arg(long, default_value = "0")]
        offset: u32,
    },
    /// Get a PnP workflow by ID
    Get {
        /// Workflow ID
        id: String,
    },
    /// Show total PnP workflow count
    Count,
}

#[derive(Debug, Subcommand)]
pub enum DiagnosticCommands {
    /// List diagnostic validation workflows
    List {
        /// Filter by run status: PENDING, IN_PROGRESS, COMPLETED, FAILED
        #[arg(long)]
        status: Option<String>,
        /// Max results to return (default: 50)
        #[arg(long, default_value = "50")]
        limit: u32,
        /// Result offset, 1-based (default: 1 = first page)
        #[arg(long, default_value = "1")]
        offset: u32,
    },
    /// Get a diagnostic workflow by ID
    Get {
        /// Workflow ID
        id: String,
    },
    /// Show total diagnostic workflow count
    Count,
    /// Submit a new diagnostic validation workflow run
    Run {
        /// Name for this workflow run
        #[arg(long)]
        name: String,
        /// Optional description
        #[arg(long)]
        description: Option<String>,
        /// Comma-separated validation set IDs to include
        #[arg(long)]
        validation_sets: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReplacementCommands {
    /// List all device replacement workflows and their status
    List,
    /// Deploy a device replacement workflow
    Deploy {
        /// Serial number of the faulty device
        #[arg(long)]
        faulty_serial: String,
        /// Serial number of the replacement device
        #[arg(long)]
        replacement_serial: String,
    },
}
