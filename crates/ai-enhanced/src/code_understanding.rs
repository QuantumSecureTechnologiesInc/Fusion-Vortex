//! Code understanding and analysis module

use crate::{AIProvider, CompletionOptions, Message, MessageRole};
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;

pub struct CodeAnalyzer {
    provider: Arc<dyn AIProvider + Send + Sync>,
}

impl CodeAnalyzer {
    pub fn new(provider: Arc<dyn AIProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    /// Explain a code file or snippet
    pub async fn explain_code(&self, code: &str, language: &str) -> Result<String> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are an expert code explainer. Provide clear, concise explanations of code functionality, structure, and design patterns.".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Explain this {} code:\n\n```{}\n{}\n```", language, language, code),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;
        Ok(response.content)
    }

    /// Analyze code for potential issues
    pub async fn analyze_issues(&self, code: &str, language: &str) -> Result<Vec<CodeIssue>> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a code analysis expert. Identify bugs, security vulnerabilities, performance issues, and code smells. Return findings as JSON array with format: [{\"severity\": \"high|medium|low\", \"type\": \"bug|security|performance|style\", \"line\": number, \"message\": \"description\"}]".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Analyze this {} code for issues:\n\n```{}\n{}\n```", language, language, code),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;

        // Parse JSON response
        let issues: Vec<CodeIssue> =
            serde_json::from_str(&response.content).unwrap_or_else(|_| Vec::new());

        Ok(issues)
    }

    /// Generate documentation for code  
    pub async fn generate_documentation(&self, code: &str, language: &str) -> Result<String> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a technical documentation expert. Generate comprehensive documentation including function/class descriptions, parameters, return values, examples, and usage notes.".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Generate documentation for this {} code:\n\n```{}\n{}\n```", language, language, code),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;
        Ok(response.content)
    }

    /// Suggest optimizations
    pub async fn suggest_optimizations(
        &self,
        code: &str,
        language: &str,
    ) -> Result<Vec<Optimization>> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a performance optimization expert. Suggest code improvements for better performance, memory usage, and efficiency. Return as JSON array with format: [{\"type\": \"performance|memory|algorithm\", \"description\": \"...\", \"improvement\": \"...\"}]".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Suggest optimizations for this {} code:\n\n```{}\n{}\n```", language, language, code),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;

        let optimizations: Vec<Optimization> =
            serde_json::from_str(&response.content).unwrap_or_else(|_| Vec::new());

        Ok(optimizations)
    }

    /// Answer questions about code
    pub async fn answer_question(
        &self,
        code: &str,
        language: &str,
        question: &str,
    ) -> Result<String> {
        let messages = vec![
            Message {
                role: MessageRole::System,
                content: "You are a helpful code assistant. Answer questions about code accurately and concisely.".to_string(),
                tool_calls: None,
            },
            Message {
                role: MessageRole::User,
                content: format!("Given this {} code:\n\n```{}\n{}\n```\n\nQuestion: {}", language, language, code, question),
                tool_calls: None,
            },
        ];

        let response = self
            .provider
            .chat_completion(messages, CompletionOptions::default())
            .await?;
        Ok(response.content)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeIssue {
    pub severity: IssueSeverity,
    #[serde(rename = "type")]
    pub issue_type: IssueType,
    pub line: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueType {
    Bug,
    Security,
    Performance,
    Style,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Optimization {
    #[serde(rename = "type")]
    pub optimization_type: String,
    pub description: String,
    pub improvement: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_issue_deserialization() {
        let json = r#"{"severity": "high", "type": "security", "line": 10, "message": "SQL injection risk"}"#;
        let issue: CodeIssue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.line, Some(10));
    }
}
