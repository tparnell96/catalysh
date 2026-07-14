use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum SiteCommands {
    /// List all sites
    List,
    /// Show site health for a specific scope or selector
    Health {
        #[command(subcommand)]
        filter: SiteHealthFilter,
    },
}

#[derive(Debug, Subcommand)]
pub enum SiteHealthFilter {
    /// Show all site health (area level)
    All,
    /// Show building-level health
    Building {
        /// Optional building name filter
        name: Option<String>,
    },
    /// Show floor-level health
    Floor {
        /// Optional floor name or hierarchy filter
        name: Option<String>,
    },
    /// Show health for a specific site by name or hierarchy
    By {
        /// Site name or full hierarchy path
        selector: String,
    },
}
