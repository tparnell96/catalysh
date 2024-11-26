use clap::{Parser, Subcommand};
use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use clap_repl::ClapEditor;
use log::error;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

mod api {
    pub mod auth;
    pub mod devices;
}
mod config;
mod utils;
mod update;

// Main command structure
#[derive(Debug, Parser)]
#[command(name = "catsh", about = "A REPL CLI for managing network devices")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Commands structure with subcommands
#[derive(Debug, Subcommand)]
enum Commands {
    Devices {
        #[command(subcommand)]
        subcommand: DeviceSubcommands,
    },
    Config {
        #[arg(long)]
        reset: bool,
    },
    Update,
    Exit,
}

// Device-specific subcommands
#[derive(Debug, Subcommand)]
enum DeviceSubcommands {
    All,
    Details {
        #[arg(help = "Device ID or Name")]
        device_id: String,
    },
}

fn get_installation_dir() -> PathBuf {
    let home = home_dir().expect("Failed to determine the user's home directory");
    home.join(".catsh")
}

fn perform_first_time_installation() -> Result<(), Box<dyn std::error::Error>> {
    let install_dir = get_installation_dir();

    if !install_dir.exists() {
        println!("Running first-time installation...");
        fs::create_dir_all(&install_dir)?;
        fs::write(install_dir.join("version"), "1.0.0")?;
        println!("First-time installation complete.");
    }
    Ok(())
}

#[allow(non_snake_case)]
fn main() {
    env_logger::init();

    if let Err(e) = perform_first_time_installation() {
        eprintln!("Error during installation: {}", e);
        return;
    }

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("catsh".to_owned()),
        ..DefaultPrompt::default()
    };

    // Create the REPL
    let rl = ClapEditor::<Cli>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "/tmp/catsh-cli-history".into()).unwrap(),
            ))
        })
        .build();

    rl.repl(|cli| {
        match cli.command {
            Commands::Devices { subcommand } => handle_devices(subcommand),
            Commands::Config { reset } => handle_config(reset),
            Commands::Update => handle_update(),
            Commands::Exit => {
                println!("Exiting catsh...");
                std::process::exit(0);
            }
        }
    });
}

fn handle_devices(subcommand: DeviceSubcommands) {
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

        match subcommand {
            DeviceSubcommands::All => {
                match api::devices::get_all_devices(&config, &token).await {
                    Ok(devices) => utils::print_devices(devices),
                    Err(e) => error!("Failed to retrieve devices: {}", e),
                }
            }
            DeviceSubcommands::Details { device_id } => {
                println!("Retrieving details for device: {}", device_id);
                // Add detailed device retrieval logic here
            }
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
}
