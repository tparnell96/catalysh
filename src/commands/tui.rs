use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TuiCommands {
    /// Launch the interactive health dashboard
    Health,
}
