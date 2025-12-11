pub mod agent;
pub mod builtin;
pub mod runtime;
pub mod scheduler;

pub use agent::{
    Agent, AgentMessage, AgentMetadata, AgentResult, AgentState, AgentStatus, AgentTask, Capability,
};
pub use builtin::{
    BugFixerAgent, CodeReviewerAgent, DocWriterAgent, RefactoringAgent, TestGeneratorAgent,
};
pub use runtime::AgentRuntime;

/// Agent library version
pub const AGENTS_VERSION: &str = env!("CARGO_PKG_VERSION");
