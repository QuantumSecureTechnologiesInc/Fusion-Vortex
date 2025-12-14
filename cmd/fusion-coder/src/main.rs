//! Fusion VSC CLI Coder
//!
//! Advanced agent orchestration CLI combining:
//! - Antigravity IDE's Planning/Fast modes
//! - Claude Code's hierarchical settings
//! - Codex's interactive workflows
//! - Full-screen TUI with live notifications

use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod exec_mode;
mod interactive;
mod tui;

#[derive(Parser)]
#[command(name = "fusion-coder")]
#[command(about = "Fusion VSC CLI Coder - Advanced agent orchestration", long_about = None)]
#[command(version)]
struct Cli {
    /// Agent mode: planning or fast
    #[arg(long, default_value = "planning")]
    mode: String,

    /// Enable secure mode
    #[arg(long)]
    secure: bool,

    /// Enable web search
    #[arg(long)]
    search: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Resume a previous session
    Resume {
        /// Session ID to resume
        session_id: Option<String>,

        /// Resume the last session
        #[arg(long)]
        last: bool,
    },

    /// Execute in non-interactive mode
    Exec {
        /// Task to execute
        task: String,

        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Generate shell completions
    Completion {
        /// Shell type
        shell: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("fusion_coder=debug")
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Resume { session_id, last }) => {
            // Resume session logic
            println!("Resume mode - not yet implemented");
        }
        Some(Commands::Exec { task, json }) => {
            // Exec mode logic
            println!("Exec mode - not yet implemented");
        }
        Some(Commands::Completion { shell }) => {
            // Generate completions
            println!("Completions - not yet implemented");
        }
        None => {
            // Interactive mode (default)
            println!("🚀 Fusion VSC CLI Coder v1.0.0");
            println!("Starting in {} mode...", cli.mode);
            if cli.secure {
                println!("🔒 Secure mode enabled");
            }
            // TODO: Launch TUI
        }
    }

    Ok(())
}
