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
        #[command(subcommand)]
        action: SetVerifySslAction,
    },
    /// Reset stored credentials (prompts for new ones if store-on-device mode is active)
    ResetCredentials,
    /// Choose how credentials are stored between sessions
    SetCredentialMode {
        #[command(subcommand)]
        mode: CredentialModeAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum SetVerifySslAction {
    /// Enable SSL verification
    Enable,
    /// Disable SSL verification
    Disable,
}

#[derive(Debug, Subcommand)]
pub enum CredentialModeAction {
    /// Encrypt and store credentials on this device (default)
    Store,
    /// Never persist credentials; prompt for password each session
    Session,
}
