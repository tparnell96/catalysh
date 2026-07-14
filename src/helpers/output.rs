// src/helpers/output.rs
// Global output format selection — set once at startup, read by print helpers.

use std::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable table (default)
    Table,
    /// Raw JSON — suitable for piping to jq or other tools
    Json,
}

thread_local! {
    static FORMAT: Cell<OutputFormat> = const { Cell::new(OutputFormat::Table) };
}

pub fn set(fmt: OutputFormat) {
    FORMAT.with(|f| f.set(fmt));
}

pub fn get() -> OutputFormat {
    FORMAT.with(|f| f.get())
}

pub fn is_json() -> bool {
    get() == OutputFormat::Json
}

/// Serialize `value` as pretty JSON to stdout, or print an error on failure.
pub fn print_json<T: serde::Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("JSON serialization error: {}", e),
    }
}
