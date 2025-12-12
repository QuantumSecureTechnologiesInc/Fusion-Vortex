//! Tab completion engine

use crate::{context::CliContext, CliError, Result};
use serde::{Deserialize, Serialize};

/// A completion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResult {
    /// The completion text
    pub text: String,

    /// Description/help text
    pub description: String,

    /// Type of completion
    pub kind: CompletionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CompletionKind {
    Command,
    Flag,
    Argument,
    Path,
    Value,
}

/// Tab completion engine
pub struct CompletionEngine {
    commands: Vec<String>,
    common_flags: Vec<String>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self {
            commands: vec![
                "build".to_string(),
                "run".to_string(),
                "test".to_string(),
                "clean".to_string(),
                "install".to_string(),
                "update".to_string(),
                "help".to_string(),
                "config".to_string(),
            ],
            common_flags: vec![
                "--release".to_string(),
                "--debug".to_string(),
                "--verbose".to_string(),
                "--quiet".to_string(),
                "--help".to_string(),
                "--version".to_string(),
            ],
        }
    }

    /// Generate completions for the given input
    pub fn complete(
        &self,
        partial: &str,
        _position: usize,
        _ctx: &CliContext,
    ) -> Result<Vec<CompletionResult>> {
        let mut completions = Vec::new();

        let parts: Vec<&str> = partial.split_whitespace().collect();

        if parts.is_empty() || (parts.len() == 1 && !partial.ends_with(' ')) {
            // Complete command names
            let prefix = parts.first().copied().unwrap_or("");
            for cmd in &self.commands {
                if cmd.starts_with(prefix) {
                    completions.push(CompletionResult {
                        text: cmd.clone(),
                        description: format!("{} command", cmd),
                        kind: CompletionKind::Command,
                    });
                }
            }
        } else {
            // Complete flags
            let last = parts.last().copied().unwrap_or("");
            if last.starts_with("--") {
                for flag in &self.common_flags {
                    if flag.starts_with(last) {
                        completions.push(CompletionResult {
                            text: flag.clone(),
                            description: format!("{} flag", flag),
                            kind: CompletionKind::Flag,
                        });
                    }
                }
            }
        }

        Ok(completions)
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}
