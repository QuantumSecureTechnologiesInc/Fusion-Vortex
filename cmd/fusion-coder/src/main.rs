//! Fusion VSC CLI Coder
//!
//! Advanced agent orchestration CLI combining:
//! - Antigravity IDE's Planning/Fast modes
//! - Claude Code's hierarchical settings
//! - Codex's interactive workflows
//! - Full-screen TUI with live notifications

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod exec_mode;
mod interactive;
mod tui;

use fusion_agent_core::{AgentModeType, AgentSession, SecureMode};
use fusion_settings::Settings;

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

    /// Model to use
    #[arg(long)]
    model: Option<String>,

    /// Working directory
    #[arg(long)]
    path: Option<PathBuf>,

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
        /// Shell type (bash, zsh, fish)
        shell: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fusion_coder=info".into()),
        )
        .init();

    let cli = Cli::parse();

    // Parse agent mode
    let mode_type: AgentModeType = cli.mode.parse()?;

    // Load settings
    let settings = Settings::load().unwrap_or_default();

    // Create secure mode
    let secure_mode = SecureMode::new(cli.secure);

    // Get workspace directory
    let workspace_dir = cli
        .path
        .unwrap_or_else(|| std::env::current_dir().expect("Failed to get current directory"));

    match cli.command {
        Some(Commands::Resume { session_id, last }) => {
            commands::handle_resume(session_id, last);
        }
        Some(Commands::Exec { task, json }) => {
            exec_mode::execute(&task, json);
        }
        Some(Commands::Completion { shell }) => {
            println!("Generating {} completions - not yet implemented", shell);
        }
        None => {
            // Interactive mode (default)
            println!("🚀 Fusion VSC CLI Coder v1.0.0");
            println!("Mode: {}", mode_type);
            if cli.secure {
                println!("🔒 Secure mode: ENABLED");
            }
            if let Some(model) = cli.model {
                println!("Model: {}", model);
            }
            println!("Workspace: {}", workspace_dir.display());
            println!();

            // Create session
            let session = AgentSession::new(mode_type, workspace_dir);
            println!("Session ID: {}", session.id);
            println!();

            // TODO: Launch TUI
            println!("Interactive TUI mode - not yet fully implemented");
            println!("Agent ready in {} mode", mode_type);
        }
    }

    Ok(())
}
