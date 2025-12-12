//! Enhanced AI capabilities with Claude Code, Codex, and Gemini-level features
//!
//! This module provides advanced AI capabilities matching and exceeding those of:
//! - Claude Code (Anthropic)
//! - GitHub Copilot / Codex (OpenAI)
//! - Gemini CLI (Google)

pub mod code_generation;
pub mod code_refactoring;
pub mod code_review;
pub mod code_understanding;
pub mod context_engine;
pub mod interactive_agent;
pub mod providers;
pub mod tool_use;

use anyhow::Result;
use std::sync::Arc;

pub use code_generation::CodeGenerator;
pub use code_refactoring::CodeRefactorer;
pub use code_review::CodeReviewer;
pub use code_understanding::CodeAnalyzer;
pub use context_engine::ContextEngine;
pub use interactive_agent::InteractiveAgent;
pub use providers::{AnthropicProvider, GeminiProvider, LocalProvider, OpenAIProvider};
pub use tool_use::ToolExecutor;

/// Main AI engine that coordinates all capabilities
pub struct EnhancedAIEngine {
    provider: Arc<dyn AIProvider + Send + Sync>,
    code_analyzer: CodeAnalyzer,
    code_generator: CodeGenerator,
    code_refactorer: CodeRefactorer,
    code_reviewer: CodeReviewer,
    context_engine: ContextEngine,
    tool_executor: ToolExecutor,
}

/// AI provider trait
#[async_trait::async_trait]
pub trait AIProvider {
    /// Get provider name
    fn name(&self) -> &str;

    /// Complete a chat conversation
    async fn chat_completion(
        &self,
        messages: Vec<Message>,
        options: CompletionOptions,
    ) -> Result<ChatCompletion>;

    /// Stream a chat completion
    async fn chat_completion_stream(
        &self,
        messages: Vec<Message>,
        options: CompletionOptions,
    ) -> Result<Box<dyn futures::Stream<Item = Result<ChatDelta>> + Unpin + Send>>;

    /// Check if provider supports tool use
    fn supports_tool_use(&self) -> bool {
        false
    }

    /// Check if provider supports vision
    fn supports_vision(&self) -> bool {
        false
    }
}

/// Chat message
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Completion options
#[derive(Debug, Clone)]
pub struct CompletionOptions {
    pub temperature: f32,
    pub max_tokens: Option<usize>,
    pub tools: Vec<Tool>,
    pub stream: bool,
}

impl Default for CompletionOptions {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: None,
            tools: Vec::new(),
            stream: false,
        }
    }
}

/// Tool definition
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Chat completion response
#[derive(Debug, Clone)]
pub struct ChatCompletion {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: FinishReason,
    pub usage: TokenUsage,
}

/// Chat delta for streaming
#[derive(Debug, Clone)]
pub struct ChatDelta {
    pub content: Option<String>,
    pub tool_call: Option<ToolCall>,
}

#[derive(Debug, Clone)]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
}

#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

impl EnhancedAIEngine {
    /// Create a new AI engine with the specified provider
    pub fn new(provider: Arc<dyn AIProvider + Send + Sync>) -> Self {
        Self {
            provider: Arc::clone(&provider),
            code_analyzer: CodeAnalyzer::new(Arc::clone(&provider)),
            code_generator: CodeGenerator::new(Arc::clone(&provider)),
            code_refactorer: CodeRefactorer::new(Arc::clone(&provider)),
            code_reviewer: CodeReviewer::new(Arc::clone(&provider)),
            context_engine: ContextEngine::new(),
            tool_executor: ToolExecutor::new(),
        }
    }

    /// Get code analyzer
    pub fn code_analyzer(&self) -> &CodeAnalyzer {
        &self.code_analyzer
    }

    /// Get code generator
    pub fn code_generator(&self) -> &CodeGenerator {
        &self.code_generator
    }

    /// Get code refactorer
    pub fn code_refactorer(&self) -> &CodeRefactorer {
        &self.code_refactorer
    }

    /// Get code reviewer
    pub fn code_reviewer(&self) -> &CodeReviewer {
        &self.code_reviewer
    }

    /// Get context engine
    pub fn context_engine(&self) -> &ContextEngine {
        &self.context_engine
    }

    /// Get tool executor
    pub fn tool_executor(&self) -> &ToolExecutor {
        &self.tool_executor
    }

    /// Create an interactive agent
    pub fn create_interactive_agent(&self) -> InteractiveAgent {
        InteractiveAgent::new(
            Arc::clone(&self.provider),
            self.context_engine.clone(),
            self.tool_executor.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_options_default() {
        let options = CompletionOptions::default();
        assert_eq!(options.temperature, 0.7);
        assert_eq!(options.stream, false);
    }
}
