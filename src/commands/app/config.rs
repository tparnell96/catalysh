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
    /// Set SSL verification
    SetVerifySsl {
        #[command(subcommand)]
        action: SetVerifySslAction,
    },
    ResetCredentials,
}

#[derive(Debug, Subcommand)]
pub enum SetVerifySslAction {
    /// Enable SSL verification
    Enable,
    /// Disable SSL verification  
    Disable,
}

