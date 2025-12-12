//! Agentic CLI integration

use crate::{CliError, Result};
#[cfg(feature = "agentic-integration")]
use fusion_agentic_core::AgenticCore;

/// Agentic CLI wrapper
pub struct AgenticCli {
    #[cfg(feature = "agentic-integration")]
    core: AgenticCore,
}

impl AgenticCli {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "agentic-integration")]
            core: AgenticCore::new(),
        }
    }

    /// Execute a command with intelligent reasoning
    pub fn execute_intelligently(&self, command: &str) -> Result<String> {
        #[cfg(feature = "agentic-integration")]
        {
            self.core
                .process_problem(&format!("Execute CLI command: {}", command))
                .map_err(|e| CliError::AgenticError(e.to_string()))
        }

        #[cfg(not(feature = "agentic-integration"))]
        {
            let _ = command;
            Err(CliError::AgenticError(
                "Agentic integration not enabled".to_string(),
            ))
        }
    }
}

impl Default for AgenticCli {
    fn default() -> Self {
        Self::new()
    }
}
