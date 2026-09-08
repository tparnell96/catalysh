mod api;
mod app;
mod helpers;

mod commands;
mod handlers;
mod tui;

use clap::Parser;
use clap_repl::ClapEditor;
use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use commands::{Cli, route_command};
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
        fs::write(install_dir.join("version"), env!("CARGO_PKG_VERSION"))?;
        println!("First-time installation complete.");
    }
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(e) = perform_first_time_installation() {
        eprintln!("Error during installation: {}", e);
        return;
    }

    // If arguments are passed, execute as a one-shot command and exit —
    // matching the nslookup pattern: no args = interactive REPL.
    if std::env::args().len() > 1 {
        match Cli::try_parse() {
            Ok(cli) => route_command(cli),
            Err(e) => e.exit(),
        }
        return;
    }

    // No arguments → enter interactive REPL
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("catalysh".to_owned()),
        ..DefaultPrompt::default()
    };

    let rl = ClapEditor::<Cli>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            let history_file = get_installation_dir().join("history");
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, history_file).unwrap(),
            ))
        })
        .build();

    rl.repl(|cli| {
        route_command(cli);
    });
}
