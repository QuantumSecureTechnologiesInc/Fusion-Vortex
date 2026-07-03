use fusion_agent_graph::{AgentGraph, AgentState};
use fusion_mcp_spec::McpRequest;
use fusion_policy::Policy;
use fusion_runtime::{Runtime, RuntimeError};
use std::collections::HashMap;

/// Agent runtime errors
#[derive(Debug)]
pub enum AgentRuntimeError {
    /// Underlying runtime error
    RuntimeError(RuntimeError),
    /// Budget exhausted
    BudgetExhausted { remaining: f64, required: f64 },
    /// Agent is not in a valid state for execution
    InvalidState(AgentState),
}

impl std::fmt::Display for AgentRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRuntimeError::RuntimeError(e) => write!(f, "Agent runtime error: {}", e),
            AgentRuntimeError::BudgetExhausted {
                remaining,
                required,
            } => {
                write!(
                    f,
                    "Budget exhausted: {} remaining, {} required",
                    remaining, required
                )
            }
            AgentRuntimeError::InvalidState(state) => {
                write!(f, "Invalid agent state for execution: {:?}", state)
            }
        }
    }
}

impl std::error::Error for AgentRuntimeError {}

impl From<RuntimeError> for AgentRuntimeError {
    fn from(e: RuntimeError) -> Self {
        AgentRuntimeError::RuntimeError(e)
    }
}

/// Agent runtime orchestrating agent execution with cost budgeting
///
/// **Key Principle**: Cost budgeting is enforced at runtime, not advisory.
pub struct AgentRuntime<P: Policy> {
    runtime: Runtime<P>,
    budget_remaining: f64,
    shared_tools: HashMap<String, u64>,
}

impl<P: Policy> AgentRuntime<P> {
    /// Create a new agent runtime
    ///
    /// # Arguments
    /// * `runtime` - The underlying Phase 1 runtime
    /// * `initial_budget` - Initial cost budget (hard limit)
    pub fn new(runtime: Runtime<P>, initial_budget: f64) -> Self {
        Self {
            runtime,
            budget_remaining: initial_budget,
            shared_tools: HashMap::new(),
        }
    }

    /// Execute an agent graph
    ///
    /// Executes steps sequentially, checking budget before each step.
    /// If budget is exhausted, the graph is paused automatically.
    ///
    /// # Errors
    /// Returns error if:
    /// - Budget is exhausted mid-execution
    /// - Policy denies a tool execution
    /// - Ledger operation fails
    pub fn execute_graph(&mut self, graph: &mut AgentGraph) -> Result<(), AgentRuntimeError> {
        while let Some(step) = graph.next_step().cloned() {
            // Budget enforcement - hard limit, not advisory
            if step.cost_estimate > self.budget_remaining {
                graph.pause();
                return Err(AgentRuntimeError::BudgetExhausted {
                    remaining: self.budget_remaining,
                    required: step.cost_estimate,
                });
            }

            // Convert PlanStep to McpRequest
            let req = McpRequest {
                id: step.id.clone(),
                tool: step.tool.clone(),
                input: step.input.clone(),
            };

            // Execute through Phase 1 runtime
            self.runtime.execute(req)?;

            // Update budget and tracking
            self.budget_remaining -= step.cost_estimate;
            self.track_tool_usage(&step.tool);

            // Advance the graph
            graph.advance();
        }

        Ok(())
    }

    /// Get the remaining budget
    pub fn budget_remaining(&self) -> f64 {
        self.budget_remaining
    }

    /// Add budget to the runtime
    pub fn add_budget(&mut self, amount: f64) {
        self.budget_remaining += amount;
    }

    /// Track tool usage for multi-agent coordination
    fn track_tool_usage(&mut self, tool: &str) {
        *self.shared_tools.entry(tool.to_string()).or_insert(0) += 1;
    }

    /// Get usage count for a specific tool
    pub fn tool_usage(&self, tool: &str) -> u64 {
        *self.shared_tools.get(tool).unwrap_or(&0)
    }

    /// Get all tool usage statistics
    pub fn all_tool_usage(&self) -> &HashMap<String, u64> {
        &self.shared_tools
    }

    /// Get a reference to the underlying runtime
    pub fn runtime(&self) -> &Runtime<P> {
        &self.runtime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusion_agent_spec::{AgentPlan, PlanStep};
    use fusion_ledger::Ledger;
    use fusion_policy::AllowListPolicy;
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_budget_enforcement() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
        let runtime = Runtime::new(ledger, policy).unwrap();
        let mut agent_runtime = AgentRuntime::new(runtime, 5.0); // Only 5.0 budget

        let mut plan = AgentPlan::new("test", "Test budget");
        plan.add_step(PlanStep::new("step1", "tool1", json!({}), "Step 1", 2.0));
        plan.add_step(PlanStep::new("step2", "tool2", json!({}), "Step 2", 4.0)); // Exceeds remaining budget

        let mut graph = AgentGraph::new(plan);

        let result = agent_runtime.execute_graph(&mut graph);

        // Should fail due to budget exhaustion
        assert!(result.is_err());

        // Graph should be paused
        assert_eq!(graph.state(), AgentState::Paused);

        // Only first step should have executed
        assert_eq!(graph.cursor(), 1);
    }

    #[test]
    fn test_successful_execution() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
        let runtime = Runtime::new(ledger, policy).unwrap();
        let mut agent_runtime = AgentRuntime::new(runtime, 10.0);

        let mut plan = AgentPlan::new("test", "Test execution");
        plan.add_step(PlanStep::new("step1", "tool1", json!({}), "Step 1", 2.0));
        plan.add_step(PlanStep::new("step2", "tool2", json!({}), "Step 2", 3.0));

        let mut graph = AgentGraph::new(plan);

        let result = agent_runtime.execute_graph(&mut graph);

        assert!(result.is_ok());
        assert_eq!(graph.state(), AgentState::Completed);
        assert_eq!(agent_runtime.budget_remaining(), 5.0);
    }

    #[test]
    fn test_tool_usage_tracking() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
        let runtime = Runtime::new(ledger, policy).unwrap();
        let mut agent_runtime = AgentRuntime::new(runtime, 20.0);

        let mut plan = AgentPlan::new("test", "Test tracking");
        plan.add_step(PlanStep::new("step1", "tool1", json!({}), "Step 1", 1.0));
        plan.add_step(PlanStep::new("step2", "tool1", json!({}), "Step 2", 1.0));
        plan.add_step(PlanStep::new("step3", "tool2", json!({}), "Step 3", 1.0));

        let mut graph = AgentGraph::new(plan);

        agent_runtime.execute_graph(&mut graph).unwrap();

        assert_eq!(agent_runtime.tool_usage("tool1"), 2);
        assert_eq!(agent_runtime.tool_usage("tool2"), 1);
        assert_eq!(agent_runtime.tool_usage("tool3"), 0);
    }

    #[test]
    fn test_budget_addition() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into()]);
        let runtime = Runtime::new(ledger, policy).unwrap();
        let mut agent_runtime = AgentRuntime::new(runtime, 5.0);

        assert_eq!(agent_runtime.budget_remaining(), 5.0);

        agent_runtime.add_budget(10.0);
        assert_eq!(agent_runtime.budget_remaining(), 15.0);
    }
}
