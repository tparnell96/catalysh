// src/helpers/command_utils.rs
// Common utilities for command handlers

use log::error;
use crate::app::config::{self, Config};
use crate::api::authentication::auth::{self, Token};
use anyhow::{Context, Result};

/// Context for executing commands that need API access
pub struct CommandContext {
    pub config: Config,
    pub token: Token,
}

impl CommandContext {
    /// Create a new command context with authentication
    pub async fn new() -> Result<Self> {
        let config = config::load_config()
            .context("Failed to load configuration")?;

        let token = auth::authenticate(&config)
            .await
            .context("Authentication failed")?;

        Ok(CommandContext { config, token })
    }
}

/// Execute an async command with proper runtime and error handling
pub fn execute_async_command<F, Fut>(f: F)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        if let Err(e) = f().await {
            error!("Command execution failed: {}", e);
        }
    });
}

/// Execute an async command that requires API context
pub fn execute_with_context<F, Fut>(f: F)
where
    F: FnOnce(CommandContext) -> Fut,
    Fut: std::future::Future<Output = Result<()>>,
{
    execute_async_command(|| async {
        let ctx = CommandContext::new().await?;
        f(ctx).await
    });
}
