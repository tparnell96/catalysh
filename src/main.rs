mod app;
mod helpers;
mod api;

mod commands;
mod handlers;

use commands::{Cli, route_command};
use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use clap_repl::ClapEditor;
use dirs::home_dir;
use std::fs;
use std::path::PathBuf;

fn get_installation_dir() -> PathBuf {
    let home = home_dir().expect("Failed to determine the user's home directory");
    home.join(".catalysh")
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
    // Initial check to confirm program is correctly installed
    if let Err(e) = perform_first_time_installation() {
        eprintln!("Error during installation: {}", e);
        return;
    }

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("catalysh".to_owned()),
        ..DefaultPrompt::default()
    };

    // Create the REPL
    let rl = ClapEditor::<Cli>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "/tmp/catalysh-cli-history".into()).unwrap(),
            ))
        })
        .build();

    rl.repl(|cli| {
        route_command(cli.command);
    });
}

