use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use clap_repl::ClapEditor;
use clap::{Parser, Subcommand};

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
                FileBackedHistory::with_file(10000, "/tmp/catalysh-config-cli-history".into()).unwrap(),
            ))
        })
        .build();

    rl.repl(|cli| {
        match cli.command {
            ConfigCommands::Dummy => {
                println!("Dummy command executed in config mode.");
            }
            ConfigCommands::Exit | ConfigCommands::End => {
                println!("Exiting configuration mode.");
                std::process::exit(0);
            }
        }
    });
}

