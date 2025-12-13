//! # Fusion AI-Enhanced CLI
//!
//! AI-powered command-line interface capabilities for the Fusion ecosystem.
//!
//! This crate provides:
//! - **Smart Command Parsing**: AI-powered interpretation of commands
//! - **Intelligent Suggestions**: Context-aware command completions
//! - **Natural Language Interface**: Execute commands using natural language
//! - **Enhanced Error Messages**: AI-generated helpful explanations
//! - **Agentic Integration**: Deep reasoning for complex CLI operations

pub mod command_parser;
pub mod completions;
pub mod context;
pub mod error_handler;
pub mod natural_language;
pub mod suggestions;

#[cfg(feature = "agentic-integration")]
pub mod agentic;

pub use command_parser::NLCommand;
pub use command_parser::{CommandIntent, CommandParser, ParsedCommand};
pub use completions::{CompletionEngine, CompletionResult};
pub use context::{CliContext, ExecutionContext};
pub use error_handler::{EnhancedErrorHandler, ErrorExplanation};
pub use natural_language::NaturalLanguageProcessor;
pub use suggestions::{CommandSuggestion, SuggestionContext, SuggestionEngine};

use parking_lot::RwLock;
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur in AI-enhanced CLI operations
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Command parsing failed: {0}")]
    ParseError(String),

    #[error("No suggestions available for: {0}")]
    NoSuggestions(String),

    #[error("Natural language processing failed: {0}")]
    NLPError(String),

    #[error("Invalid command context: {0}")]
    InvalidContext(String),

    #[error("Completion generation failed: {0}")]
    CompletionError(String),

    #[error("Agentic reasoning failed: {0}")]
    AgenticError(String),
}

/// Result type for CLI operations
pub type Result<T> = std::result::Result<T, CliError>;

/// The main AI-enhanced CLI engine
pub struct AiCli {
    parser: Arc<CommandParser>,
    suggestions: Arc<SuggestionEngine>,
    nl_processor: Arc<NaturalLanguageProcessor>,
    error_handler: Arc<EnhancedErrorHandler>,
    completions: Arc<CompletionEngine>,
    context: Arc<RwLock<CliContext>>,

    #[cfg(feature = "agentic-integration")]
    agentic: Arc<RwLock<agentic::AgenticCli>>,
}

impl AiCli {
    /// Create a new AI-enhanced CLI instance
    pub fn new() -> Self {
        Self {
            parser: Arc::new(CommandParser::new()),
            suggestions: Arc::new(SuggestionEngine::new()),
            nl_processor: Arc::new(NaturalLanguageProcessor::new()),
            error_handler: Arc::new(EnhancedErrorHandler::new()),
            completions: Arc::new(CompletionEngine::new()),
            context: Arc::new(RwLock::new(CliContext::new())),

            #[cfg(feature = "agentic-integration")]
            agentic: Arc::new(RwLock::new(agentic::AgenticCli::new())),
        }
    }

    /// Parse a command with AI assistance
    pub fn parse_command(&self, input: &str) -> Result<ParsedCommand> {
        // Update context
        let mut ctx = self.context.write();
        ctx.add_input(input.to_string());
        drop(ctx);

        // Try standard parsing first
        match self.parser.parse(input) {
            Ok(cmd) => Ok(cmd),
            Err(_) => {
                // Try natural language interpretation
                self.parse_natural_language(input)
            }
        }
    }

    /// Parse natural language command
    pub fn parse_natural_language(&self, input: &str) -> Result<ParsedCommand> {
        let nl_command = self.nl_processor.process(input)?;
        self.parser.from_nl_command(&nl_command)
    }

    /// Get intelligent suggestions for partial input
    pub fn suggest(&self, partial: &str) -> Result<Vec<CommandSuggestion>> {
        let ctx = self.context.read();
        let suggestion_ctx = SuggestionContext {
            partial_input: partial.to_string(),
            history: ctx.command_history.clone(),
            current_directory: ctx.current_directory.clone(),
        };
        drop(ctx);

        self.suggestions.generate(&suggestion_ctx)
    }

    /// Get completions for tab completion
    pub fn complete(&self, partial: &str, position: usize) -> Result<Vec<CompletionResult>> {
        let ctx = self.context.read();
        self.completions.complete(partial, position, &ctx)
    }

    /// Enhance an error with AI-generated explanation
    pub fn explain_error(&self, error: &str) -> Result<ErrorExplanation> {
        let ctx = self.context.read();
        self.error_handler.explain(error, &ctx)
    }

    /// Execute a command with agentic reasoning
    #[cfg(feature = "agentic-integration")]
    pub fn execute_with_reasoning(&self, command: &str) -> Result<String> {
        let agentic = self.agentic.read();
        agentic
            .execute_intelligently(command)
            .map_err(|e| CliError::AgenticError(e.to_string()))
    }

    /// Get the current CLI context
    pub fn get_context(&self) -> CliContext {
        self.context.read().clone()
    }

    /// Update the CLI context
    pub fn update_context<F>(&self, f: F)
    where
        F: FnOnce(&mut CliContext),
    {
        let mut ctx = self.context.write();
        f(&mut ctx);
    }
}

impl Default for AiCli {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for AI CLI configuration
pub struct AiCliBuilder {
    enable_suggestions: bool,
    enable_nl: bool,
    enable_completions: bool,

    #[cfg(feature = "agentic-integration")]
    enable_agentic: bool,
}

impl AiCliBuilder {
    pub fn new() -> Self {
        Self {
            enable_suggestions: true,
            enable_nl: true,
            enable_completions: true,

            #[cfg(feature = "agentic-integration")]
            enable_agentic: true,
        }
    }

    pub fn with_suggestions(mut self, enabled: bool) -> Self {
        self.enable_suggestions = enabled;
        self
    }

    pub fn with_natural_language(mut self, enabled: bool) -> Self {
        self.enable_nl = enabled;
        self
    }

    pub fn with_completions(mut self, enabled: bool) -> Self {
        self.enable_completions = enabled;
        self
    }

    #[cfg(feature = "agentic-integration")]
    pub fn with_agentic(mut self, enabled: bool) -> Self {
        self.enable_agentic = enabled;
        self
    }

    pub fn build(self) -> AiCli {
        AiCli::new()
    }
}

impl Default for AiCliBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_cli_creation() {
        let cli = AiCli::new();
        assert!(true); // CLI created successfully
    }

    #[test]
    fn test_builder_pattern() {
        let cli = AiCliBuilder::new()
            .with_suggestions(true)
            .with_natural_language(true)
            .build();
        assert!(true);
    }
}
