use clap::{Parser, Subcommand};
use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use clap_repl::ClapEditor;
use std::panic::{catch_unwind, panic_any, set_hook, take_hook, AssertUnwindSafe};
use std::sync::Arc;

#[derive(Debug, Parser)]
#[command(name = "", about = "Configuration Mode REPL")]
struct ConfigCli {
    #[command(subcommand)]
    command: ConfigCommands,
}

#[derive(Debug, Subcommand)]
enum ConfigCommands {
    /// Dummy command for demonstration
    Dummy,
    /// Exit configuration mode
    Exit,
    /// End configuration mode
    End,
}

#[derive(Debug)]
struct ExitRepl;

pub fn start_config_repl() {
    println!("Entering configuration mode. Type 'exit' or 'end' to leave.");
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("catlysh(config)#".to_owned()),
        ..DefaultPrompt::default()
    };

    let rl = ClapEditor::<ConfigCli>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "/tmp/catalysh-config-cli-history".into())
                    .unwrap(),
            ))
        })
        .build();

    // Save the original panic hook
    let original_hook = Arc::new(take_hook());

    // Set a custom panic hook that suppresses our exit panic
    let hook_for_closure = Arc::clone(&original_hook);
    set_hook(Box::new(move |panic_info| {
        if panic_info.payload().downcast_ref::<ExitRepl>().is_none() {
            // Not our exit panic, call the original hook
            hook_for_closure(panic_info);
        }
    }));

    let result = catch_unwind(AssertUnwindSafe(|| {
        rl.repl(|cli| match cli.command {
            ConfigCommands::Dummy => {
                println!("Dummy command executed in config mode.");
            }
            ConfigCommands::Exit | ConfigCommands::End => {
                panic_any(ExitRepl);
            }
        });
    }));

    // Restore the original panic hook
    set_hook(Arc::try_unwrap(original_hook).unwrap_or_else(|_| take_hook()));

    match result {
        Ok(_) => (),
        Err(e) => {
            if e.downcast_ref::<ExitRepl>().is_some() {
                // Clean exit from config mode
                return;
            }
            // Re-panic for any other panic type
            std::panic::resume_unwind(e);
        }
    }
}
