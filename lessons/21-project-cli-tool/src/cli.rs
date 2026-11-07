use clap::{Parser, Subcommand};

/// A tiny task manager for your terminal.
#[derive(Parser, Debug)]
#[command(name = "tasks", version)]
pub struct Cli {
    /// Path to the task file
    #[arg(long, default_value = "tasks.json", global = true)]
    pub file: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new task
    Add {
        /// What needs doing
        title: Vec<String>,
        /// Priority from 1 (high) to 3 (low)
        #[arg(short, long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(1..=3))]
        priority: u8,
    },
    /// List tasks
    List {
        /// Include completed tasks
        #[arg(short, long)]
        all: bool,
    },
    /// Mark a task as done
    Done { id: u32 },
    /// Remove a task
    Remove { id: u32 },
}
