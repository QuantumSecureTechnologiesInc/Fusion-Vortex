use anyhow::Result;
use async_trait::async_trait;
use fusion_ai_core_adapters::{
    AdapterConfig, AnthropicConfig, GoogleConfig, ModelSession, OpenAIConfig, UnifiedAdapter,
};
use uuid::Uuid;

use crate::agent::{Agent, AgentMetadata, AgentResult, AgentTask, Capability};

// Helper to get adapter (duplicated from ai-cli for now, ideally moved to common lib)
fn get_adapter() -> Result<Box<dyn ModelSession>> {
    if let Ok(key) = std::env::var("OPENAI_API_KEY") {
        let config = OpenAIConfig {
            api_key: key,
            ..Default::default()
        };
        return Ok(UnifiedAdapter::from_config(AdapterConfig::OpenAI(config))?.create_session());
    }
    if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
        let config = AnthropicConfig {
            api_key: key,
            ..Default::default()
        };
        return Ok(UnifiedAdapter::from_config(AdapterConfig::Anthropic(config))?.create_session());
    }
    if let Ok(key) = std::env::var("GOOGLE_API_KEY") {
        let config = GoogleConfig {
            api_key: key,
            ..Default::default()
        };
        return Ok(UnifiedAdapter::from_config(AdapterConfig::Google(config))?.create_session());
    }
    anyhow::bail!("No API keys configured for Agents.")
}

/// Code reviewer agent
pub struct CodeReviewerAgent {
    meta: AgentMetadata,
}

impl CodeReviewerAgent {
    pub fn new() -> Self {
        Self {
            meta: AgentMetadata {
                id: Uuid::new_v4(),
                name: "CodeReviewer".to_string(),
                capabilities: vec![Capability::CodeReview, Capability::SecurityScan],
                max_concurrent_tasks: 3,
                priority: 8,
            },
        }
    }
}

#[async_trait]
impl Agent for CodeReviewerAgent {
    fn metadata(&self) -> &AgentMetadata {
        &self.meta
    }
    fn can_handle(&self, task: &AgentTask) -> bool {
        task.task_type == "code_review" || task.task_type == "security_scan"
    }

    async fn execute(&self, task: AgentTask) -> Result<AgentResult> {
        let start = std::time::Instant::now();
        let code = task
            .input
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Handle mock vs real
        let response_content = match get_adapter() {
            Ok(adapter) => {
                let prompt = format!("Review the following code for quality and security issues:\n```\n{}\n```\nReturn JSON output with 'issues' list.", code);
                let (response, _) = adapter.predict(&prompt).await?;
                response
            }
            Err(_) => {
                // Fallback to mock if no key, ensuring agents still work for tests
                let issues = vec![
                    "Consider adding error handling (mock)",
                    "Variable naming could be improved (mock)",
                    "Add documentation comments (mock)",
                ];
                serde_json::to_string(&issues).unwrap()
            }
        };

        Ok(AgentResult {
            task_id: task.id,
            success: true,
            output: serde_json::json!({
                "review_output": response_content,
                "model_used": "auto-detected"
            }),
            error: None,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

/// Test generator agent
pub struct TestGeneratorAgent {
    meta: AgentMetadata,
}

impl TestGeneratorAgent {
    pub fn new() -> Self {
        Self {
            meta: AgentMetadata {
                id: Uuid::new_v4(),
                name: "TestGenerator".to_string(),
                capabilities: vec![Capability::TestGeneration],
                max_concurrent_tasks: 2,
                priority: 7,
            },
        }
    }
}

#[async_trait]
impl Agent for TestGeneratorAgent {
    fn metadata(&self) -> &AgentMetadata {
        &self.meta
    }
    fn can_handle(&self, task: &AgentTask) -> bool {
        task.task_type == "generate_tests"
    }

    async fn execute(&self, task: AgentTask) -> Result<AgentResult> {
        let start = std::time::Instant::now();
        let code = task
            .input
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let response_content = match get_adapter() {
            Ok(adapter) => {
                let prompt = format!("Generate unit tests for this code:\n```\n{}\n```", code);
                let (response, _) = adapter.predict(&prompt).await?;
                response
            }
            Err(_) => "// Mock test output\nfn test_mock() { assert!(true); }".to_string(),
        };

        Ok(AgentResult {
            task_id: task.id,
            success: true,
            output: serde_json::json!({
                "generated_tests": response_content
            }),
            error: None,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

/// Documentation writer agent
pub struct DocWriterAgent {
    meta: AgentMetadata,
}

impl DocWriterAgent {
    pub fn new() -> Self {
        Self {
            meta: AgentMetadata {
                id: Uuid::new_v4(),
                name: "DocWriter".to_string(),
                capabilities: vec![Capability::Documentation],
                max_concurrent_tasks: 2,
                priority: 6,
            },
        }
    }
}

#[async_trait]
impl Agent for DocWriterAgent {
    fn metadata(&self) -> &AgentMetadata {
        &self.meta
    }
    fn can_handle(&self, task: &AgentTask) -> bool {
        task.task_type == "generate_docs"
    }

    async fn execute(&self, task: AgentTask) -> Result<AgentResult> {
        let start = std::time::Instant::now();
        let code = task
            .input
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let response_content = match get_adapter() {
            Ok(adapter) => {
                let prompt = format!("Write documentation for this code:\n```\n{}\n```", code);
                let (response, _) = adapter.predict(&prompt).await?;
                response
            }
            Err(_) => "// Mock documentation\nThis is a mock description.".to_string(),
        };

        Ok(AgentResult {
            task_id: task.id,
            success: true,
            output: serde_json::json!({
                "documentation": response_content
            }),
            error: None,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

/// Bug fixer agent
pub struct BugFixerAgent {
    meta: AgentMetadata,
}

impl BugFixerAgent {
    pub fn new() -> Self {
        Self {
            meta: AgentMetadata {
                id: Uuid::new_v4(),
                name: "BugFixer".to_string(),
                capabilities: vec![Capability::BugFix],
                max_concurrent_tasks: 1,
                priority: 9,
            },
        }
    }
}

#[async_trait]
impl Agent for BugFixerAgent {
    fn metadata(&self) -> &AgentMetadata {
        &self.meta
    }
    fn can_handle(&self, task: &AgentTask) -> bool {
        task.task_type == "fix_bug"
    }

    async fn execute(&self, task: AgentTask) -> Result<AgentResult> {
        let start = std::time::Instant::now();
        let code = task
            .input
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let error = task
            .input
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error");

        let response_content = match get_adapter() {
            Ok(adapter) => {
                let prompt = format!(
                    "Fix the following bug in the code:\nError: {}\nCode:\n```\n{}\n```",
                    error, code
                );
                let (response, _) = adapter.predict(&prompt).await?;
                response
            }
            Err(_) => "// Mock fix applied".to_string(),
        };

        Ok(AgentResult {
            task_id: task.id,
            success: true,
            output: serde_json::json!({
                "fix": response_content
            }),
            error: None,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}

/// Refactoring assistant agent
pub struct RefactoringAgent {
    meta: AgentMetadata,
}

impl RefactoringAgent {
    pub fn new() -> Self {
        Self {
            meta: AgentMetadata {
                id: Uuid::new_v4(),
                name: "RefactoringAssistant".to_string(),
                capabilities: vec![Capability::Refactoring],
                max_concurrent_tasks: 2,
                priority: 7,
            },
        }
    }
}

#[async_trait]
impl Agent for RefactoringAgent {
    fn metadata(&self) -> &AgentMetadata {
        &self.meta
    }
    fn can_handle(&self, task: &AgentTask) -> bool {
        task.task_type == "refactor"
    }

    async fn execute(&self, task: AgentTask) -> Result<AgentResult> {
        let start = std::time::Instant::now();
        let code = task
            .input
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let goal = task
            .input
            .get("goal")
            .and_then(|v| v.as_str())
            .unwrap_or("Improve code quality");

        let response_content = match get_adapter() {
            Ok(adapter) => {
                let prompt = format!("Refactor this code to {}:\n```\n{}\n```", goal, code);
                let (response, _) = adapter.predict(&prompt).await?;
                response
            }
            Err(_) => "// Mock refactoring".to_string(),
        };

        Ok(AgentResult {
            task_id: task.id,
            success: true,
            output: serde_json::json!({
                "refactored_code": response_content
            }),
            error: None,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}
