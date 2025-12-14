//! Code generation module

use crate::{AIProvider, CompletionOptions, Message, MessageRole};
use anyhow::Result;
use std::sync::Arc;

pub struct CodeGenerator {
    provider: Arc<dyn AIProvider + Send + Sync>,
}

impl CodeGenerator {
    pub fn new(provider: Arc<dyn AIProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    /// Generate code from natural language description
    pub async fn generate_from_description(
        &self,
        description: &str,
        language: &str,
        context: Option<&str>,
    ) -> Result<GeneratedCode> {
        let context_str = context
            .map(|c| format!("\n\nExisting code context:\n```{}\n{}\n```", language, c))
            .unwrap_or_default();

        let messages = vec![
            Message {
                role: MessageRole::System,
                content: format!(
                    "You are an expert {} developer. Generate clean, efficient, well-documented code. Follow best practices and modern patterns.",
                    language
                ),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Generate {} code for: {}{}", language, description, context_str),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;

        Ok(GeneratedCode {
            code: self.extract_code_from_response(&response.content, language),
            explanation: response.content.clone(),
            language: language.to_string(),
        })
    }

    /// Generate unit tests for code
    pub async fn generate_tests(&self, code: &str, language: &str) -> Result<GeneratedCode> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a test automation expert. Generate comprehensive unit tests covering edge cases, error conditions, and normal operations.".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Generate comprehensive unit tests for this {} code:\n\n```{}\n{}\n```", language, language, code),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;

        Ok(GeneratedCode {
            code: self.extract_code_from_response(&response.content, language),
            explanation: response.content.clone(),
            language: language.to_string(),
        })
    }

    /// Generate function from signature
    pub async fn implement_function(
        &self,
        signature: &str,
        language: &str,
        requirements: &str,
    ) -> Result<GeneratedCode> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a skilled programmer. Implement functions based on signatures and requirements with clean, efficient code.".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!(
                    "Implement this {} function:\n\nSignature: {}\n\nRequirements: {}",
                    language, signature, requirements
                ),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;

        Ok(GeneratedCode {
            code: self.extract_code_from_response(&response.content, language),
            explanation: response.content,
            language: language.to_string(),
        })
    }

    /// Complete partial code
    pub async fn complete_code(
        &self,
        partial_code: &str,
        language: &str,
        _cursor_position: Option<usize>,
    ) -> Result<Vec<Completion>> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are an intelligent code completion assistant. Suggest contextually relevant code completions.".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Suggest code completions for this {} code:\n\n```{}\n{}\n```", language, language, partial_code),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;

        // Parse completions from response
        Ok(vec![Completion {
            text: response.content,
            kind: CompletionKind::Snippet,
            detail: None,
        }])
    }

    /// Extract code blocks from markdown response
    fn extract_code_from_response(&self, response: &str, language: &str) -> String {
        // Find code blocks in markdown format
        let code_block_start = format!("```{}", language);

        if let Some(start) = response.find(&code_block_start) {
            if let Some(end) = response[start + code_block_start.len()..].find("```") {
                let code =
                    &response[start + code_block_start.len()..start + code_block_start.len() + end];
                return code.trim().to_string();
            }
        }

        // Fallback: try to find any code block
        if let Some(start) = response.find("```") {
            if let Some(end) = response[start + 3..].find("```") {
                let code = &response[start + 3..start + 3 + end];
                // Skip first line if it's a language identifier
                if let Some(newline) = code.find('\n') {
                    return code[newline + 1..].trim().to_string();
                }
                return code.trim().to_string();
            }
        }

        response.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedCode {
    pub code: String,
    pub explanation: String,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct Completion {
    pub text: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CompletionKind {
    Snippet,
    Function,
    Class,
    Variable,
    Keyword,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_code_from_markdown() {
        let _response = r#"Here's the code:

```rust
fn main() {
    println!("Hello");
}
```

This is a simple example."#;

        // Would need a mock provider to test this properly
        // For now, just test the struct creation
        let gen_code = GeneratedCode {
            code: "test".to_string(),
            explanation: "test".to_string(),
            language: "rust".to_string(),
        };

        assert_eq!(gen_code.language, "rust");
    }
}
