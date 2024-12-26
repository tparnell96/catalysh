use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum AppConfigCommands {
    /// Reset app configuration
    Reset,
    /// Show current configuration
    Show,
    /// Set DNA Center URL
    SetUrl {
        #[arg(help = "New DNA Center URL")]
        url: String,
    },
    /// Set SSL verification
    SetVerifySsl {
        #[arg(long, help = "Enable or disable SSL verification")]
        enabled: bool,
    },
    /// Reset only the stored credentials
    ResetCredentials,
}

