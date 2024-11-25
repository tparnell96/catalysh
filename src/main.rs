use std::fs;
use std::path::PathBuf;

use clap::Parser;
use clap_repl::reedline::{
    DefaultPrompt, DefaultPromptSegment, FileBackedHistory,
};
use clap_repl::ClapEditor;
use log::error;
use dirs::home_dir;

mod api {
    pub mod auth;
    pub mod devices;
}
mod config;
mod utils;
mod update;

// Main CLI structure
#[derive(Debug, Parser)]
#[command(name = "")] // Name is empty to avoid it showing in error messages
enum CliCommand {
    Devices {
        /// Get all devices
        #[arg(short, long)]
        all: bool,
    },
    Config {
        /// Reset the configuration
        #[arg(long)]
        reset: bool,
    },
    Update,
    Exit,
}

fn get_installation_dir() -> PathBuf {
    let home = home_dir().expect("Failed to determine the user's home directory");
    home.join(".catsh")
}

fn perform_first_time_installation() -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = get_installation_dir();

    if !install_dir.exists() {
        println!("Running first-time installation...");
        // Create installation directory and perform setup
        fs::create_dir_all(&install_dir)?;
        // Example: Write a version file
        fs::write(install_dir.join("version"), "1.0.0")?;
        println!("First-time installation complete.");
    } 
    

    Ok(())
}

fn main() {
    env_logger::init();

    // Perform first-time installation if necessary
    if let Err(e) = perform_first_time_installation() {
        eprintln!("Error during installation: {}", e);
        return;
    }

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("catsh".to_owned()),
        ..DefaultPrompt::default()
    };

    // Create the REPL
    let rl = ClapEditor::<CliCommand>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "/tmp/catsh-cli-history".into()).unwrap(),
            ))
        })
        .build();

    rl.repl(|command| {
        match command {
            CliCommand::Devices { all } => {
                handle_inventory(all);
            }
            CliCommand::Config { reset } => {
                handle_config(reset);
            }
            CliCommand::Update => {
                handle_update();
            }
            CliCommand::Exit => {
                println!("Exiting catsh...");
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

        let token = match api::auth::authenticate(&config).await {
            Ok(t) => t,
            Err(e) => {
                error!("Authentication failed: {}", e);
                return;
            }
        };

        if all {
            match api::devices::get_all_devices(&config, &token).await {
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
fn handle_update() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        if let Err(e) = update::update_to_latest() {
            eprintln!("Update failed: {}", e);
        } else {
            println!("Update completed successfully.");
        }
    }

    #[cfg(target_os = "windows")]
    {
        println!("Please download and run the latest `windows_installer.exe` to update the application.");
    }

    std::process::exit(0);
}
