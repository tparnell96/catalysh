use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_repl::reedline::{
    DefaultPrompt, DefaultPromptSegment, FileBackedHistory, Reedline, Signal,
};
use clap_repl::ClapEditor;
use log::error;

mod api;
mod auth;
mod config;
mod utils;

/// Main CLI structure
#[derive(Debug, Parser)]
#[command(name = "")] // Name is empty to avoid it showing in error messages
enum CliCommand {
    Inventory {
        /// Get all devices
        #[arg(short, long)]
        all: bool,
    },
    Config {
        /// Reset the configuration
        #[arg(long)]
        reset: bool,
    },
    Exit,
}

fn main() {
    env_logger::init();

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("Cat-CLI".to_owned()),
        ..DefaultPrompt::default()
    };

    // Create the REPL
    let rl = ClapEditor::<CliCommand>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "/tmp/dnac-cli-history".into()).unwrap(),
            ))
        })
        .build();

    rl.repl(|command| {
        match command {
            CliCommand::Inventory { all } => {
                handle_inventory(all);
            }
            CliCommand::Config { reset } => {
                handle_config(reset);
            }
            CliCommand::Exit => {
                println!("Exiting Catalyst Center CLI...");
                std::process::exit(0);
            }
        }
    });
}

fn handle_inventory(all: bool) {
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    runtime.block_on(async {
        let config = match config::load_config() {
            Ok(cfg) => cfg,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                return;
            }
        };

        let token = match auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };

        if all {
            match api::get_all_devices(&config, &token).await {
                Ok(devices) => utils::print_devices(devices),
                Err(e) => error!("Failed to retrieve devices: {}", e),
            }
        } else {
            println!("Use the `--all` flag to retrieve all devices.");
        }
    });
}

fn handle_config(reset: bool) {
    if reset {
        if let Err(e) = config::reset_config() {
            error!("Failed to reset configuration: {}", e);
        } else {
            println!("Configuration reset successfully.");
        }
    } else {
        println!("No valid config subcommand provided. Use `--reset` to reset the configuration.");
    }
}
