pub mod adapter;
pub mod cache;
pub mod policy;
pub mod preview;
pub mod prompt;
pub mod safety;
pub mod workspace;

// Re-export production adapters
pub use adapter::{
    AdapterConfig, Explanation, ModelSession, PredictOptions, ResponseChunk, UnifiedAdapter, Usage,
};

// Re-export adapter-specific types
pub use adapter::adapters::{
    anthropic::{AnthropicAdapter, AnthropicConfig, AnthropicMessage, ContentBlock},
    google::{GoogleAdapter, GoogleConfig, GoogleContent, GooglePart},
    openai::{OpenAIAdapter, OpenAIConfig, OpenAIMessage},
};

// Re-export other core components
pub use cache::Cache;
pub use policy::PolicyManager;
pub use preview::{ApplyMode, ApplyResult, Patch, PatchMetadata, PreviewEngine};
pub use prompt::PromptManager;
pub use safety::{SafetyEngine, SafetyIssue, SafetyIssueKind, SafetyLevel, SafetyReport};
pub use workspace::{
    Dependency, FileContext, FileType, ProjectConfig, WorkspaceContext, WorkspaceLoader,
};

/// AI Core version
pub const AI_CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!AI_CORE_VERSION.is_empty());
    }

    #[test]
    fn test_adapter_creation() {
        let config = AdapterConfig::OpenAI(OpenAIConfig {
            api_key: "test".to_string(),
            ..Default::default()
        });

        let adapter = UnifiedAdapter::from_config(config);
        assert!(adapter.is_ok());
    }
}
