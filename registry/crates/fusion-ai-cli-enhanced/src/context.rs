//! CLI execution context

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CLI execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliContext {
    /// Current working directory
    pub current_directory: String,

    /// Command history
    pub command_history: Vec<String>,

    /// Environment variables
    pub env_vars: Vec<(String, String)>,

    /// Last command result
    pub last_result: Option<String>,
}

impl CliContext {
    pub fn new() -> Self {
        Self {
            current_directory: std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("/"))
                .to_string_lossy()
                .to_string(),
            command_history: Vec::new(),
            env_vars: Vec::new(),
            last_result: None,
        }
    }

    pub fn add_input(&mut self, input: String) {
        self.command_history.push(input);

        // Keep only last 100 commands
        if self.command_history.len() > 100 {
            self.command_history.remove(0);
        }
    }

    pub fn set_last_result(&mut self, result: String) {
        self.last_result = Some(result);
    }
}

impl Default for CliContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Extended execution context with more metadata
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub cli_context: CliContext,
    pub is_interactive: bool,
    pub color_enabled: bool,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            cli_context: CliContext::new(),
            is_interactive: true,
            color_enabled: true,
        }
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self::new()
    }
}
