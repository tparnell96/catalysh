// src/commands/show/task.rs
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    /// List tasks
    List {
        /// Starting offset
        #[arg(long, default_value_t = 1)]
        offset: u32,
        /// Number of tasks to return
        #[arg(long, default_value_t = 10)]
        limit: u32,
    },
    /// Get task details by ID
    Get {
        /// Task ID
        task_id: String,
    },
}
