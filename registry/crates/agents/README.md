# Fusion Agents Framework

**Version:** Workspace
**Type:** Agentic AI Framework
**License:** MIT / Apache 2.0 Dual License

## Overview

Fusion Agents is a powerful framework for building and orchestrating autonomous AI agents capable of performing complex software engineering tasks in parallel. It provides a runtime, scheduling, and standard interfaces for agent communication.

## Core Components

- **Agent Runtime**: Manages agent lifecycle, message passing, and state persistence
- **Scheduler**: Optimizes agent execution across available CPU/GPU resources
- **Specialized Agents**: Built-in implementations for common tasks

## Built-in Agents

The framework includes several production-ready agents:

- **BugFixerAgent**: Analyzes error logs and implementation to propose fixes
- **CodeReviewerAgent**: Performs security and style audits on code changes
- **DocWriterAgent**: Generates comprehensive documentation from source code
- **RefactoringAgent**: Modernizes legacy code and improves performance
- **TestGeneratorAgent**: Automatically creates unit and integration tests

## Usage

```rust
use fusion_agents::{Agent, AgentRuntime, CodeReviewerAgent};
use fusion_ai_core::ModelConfig;

#[tokio::main]

async fn main() -> Result<(), anyhow::Error> {
    // Initialize runtime
    let mut runtime = AgentRuntime::new();

    // Create an agent
    let reviewer = CodeReviewerAgent::new(ModelConfig::default());

    // spawning agent task
    let task_id = runtime.spawn(reviewer, "src/lib.rs").await?;

    // Await result
    let report = runtime.await_result(task_id).await?;
    println!("Review complete: {:?}", report);

    Ok(())
}
```text

## Features

- **Parallel Execution**: Run multiple agents concurrently
- **Inter-Agent Communication**: Agents can collaborate and share context
- **State Management**: Automatic persistence of agent memory
- **Tool Access**: Secure access to file system, shell, and compilers
- **Observability**: Built-in tracing for debugging agent thought processes

## Integration

Integrates seamlessly with:
- **Fusion AI Core**: For model inference and safety checks
- **Fusion Runtime**: For efficient async execution

## Dependencies

- `fusion-ai-core`
- `tokio`
- `async-trait`
- `tracing`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)