use fusion_ledger::{Ledger, LedgerEntry, LedgerError};
use fusion_mcp_spec::McpRequest;
use fusion_policy::{Policy, PolicyContext, PolicyDecision};

/// Runtime errors
#[derive(Debug)]
pub enum RuntimeError {
    /// Ledger operation failed
    LedgerError(LedgerError),
    /// Policy denied execution
    PolicyDenied(String),
    /// Serialization error
    SerializationError(serde_json::Error),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::LedgerError(e) => write!(f, "Runtime ledger error: {}", e),
            RuntimeError::PolicyDenied(reason) => write!(f, "Policy denied execution: {}", reason),
            RuntimeError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<LedgerError> for RuntimeError {
    fn from(e: LedgerError) -> Self {
        RuntimeError::LedgerError(e)
    }
}

impl From<serde_json::Error> for RuntimeError {
    fn from(e: serde_json::Error) -> Self {
        RuntimeError::SerializationError(e)
    }
}

/// Crash-only runtime
///
/// **Key Principles:**
/// - No recovery logic
/// - Restart = replay
/// - Deterministic state reconstruction
/// - Kill the process at any time → restart achieves deterministic state
pub struct Runtime<P: Policy> {
    ledger: Ledger,
    policy: P,
    sequence: u64,
    context: Option<PolicyContext>,
}

impl<P: Policy> Runtime<P> {
    /// Create a new runtime with the specified ledger and policy
    ///
    /// On creation, the runtime replays the ledger to determine the current sequence number.
    ///
    /// # Errors
    /// Returns `RuntimeError` if ledger replay fails
    ///
    /// # Example
    /// ```
    /// use fusion_runtime::Runtime;
    /// use fusion_ledger::Ledger;
    /// use fusion_policy::AllowListPolicy;
    ///
    /// let ledger = Ledger::new("runtime.log");
    /// let policy = AllowListPolicy::new(vec!["safe_tool".into()]);
    /// let runtime = Runtime::new(ledger, policy).unwrap();
    /// ```
    pub fn new(ledger: Ledger, policy: P) -> Result<Self, RuntimeError> {
        let seq = ledger.len()?;
        Ok(Self {
            ledger,
            policy,
            sequence: seq,
            context: None,
        })
    }

    /// Set the policy context for more sophisticated policy evaluation
    pub fn set_context(&mut self, context: PolicyContext) {
        self.context = Some(context);
    }

    /// Get the current execution sequence number
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Execute an MCP request through the runtime
    ///
    /// **Execution Flow:**
    /// 1. Evaluate policy (pre-execution gate)
    /// 2. If allowed, append to ledger
    /// 3. Increment sequence
    /// 4. Update context if present
    ///
    /// # Errors
    /// Returns `RuntimeError::PolicyDenied` if policy denies execution
    /// Returns `RuntimeError::LedgerError` if ledger append fails
    ///
    /// # Example
    /// ```
    /// use fusion_runtime::Runtime;
    /// use fusion_ledger::Ledger;
    /// use fusion_policy::AllowListPolicy;
    /// use fusion_mcp_spec::McpRequest;
    /// use serde_json::json;
    ///
    /// let ledger = Ledger::new("test.log");
    /// let policy = AllowListPolicy::new(vec!["safe_tool".into()]);
    /// let mut runtime = Runtime::new(ledger, policy).unwrap();
    ///
    /// let req = McpRequest {
    ///     id: "1".into(),
    ///     tool: "safe_tool".into(),
    ///     input: json!({}),
    /// };
    ///
    /// runtime.execute(req).unwrap();
    /// ```
    pub fn execute(&mut self, req: McpRequest) -> Result<(), RuntimeError> {
        // Policy enforcement gate
        let decision = self.policy.evaluate(&req.tool, self.context.as_ref());

        match decision {
            PolicyDecision::Allow => {
                // Create ledger entry
                let entry = LedgerEntry {
                    sequence: self.sequence,
                    payload: serde_json::to_value(&req)?,
                };

                // Append to ledger (crash-safe)
                self.ledger.append(&entry)?;

                // Update runtime state
                self.sequence += 1;

                // Update context execution history if present
                if let Some(ref mut ctx) = self.context {
                    ctx.execution_history.push(req.tool.clone());
                }

                Ok(())
            }
            PolicyDecision::Deny(reason) => Err(RuntimeError::PolicyDenied(reason)),
        }
    }

    /// Replay the entire ledger and return all executed requests
    ///
    /// This is useful for crash recovery or audit purposes.
    ///
    /// # Errors
    /// Returns `RuntimeError` if ledger replay or deserialization fails
    pub fn replay_all(&self) -> Result<Vec<McpRequest>, RuntimeError> {
        let entries = self.ledger.replay()?;
        let requests: Result<Vec<McpRequest>, serde_json::Error> = entries
            .into_iter()
            .map(|entry| serde_json::from_value(entry.payload))
            .collect();

        Ok(requests?)
    }

    /// Get a reference to the underlying ledger
    pub fn ledger(&self) -> &Ledger {
        &self.ledger
    }

    /// Get a reference to the policy
    pub fn policy(&self) -> &P {
        &self.policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fusion_policy::AllowListPolicy;
    use serde_json::json;
    use tempfile::NamedTempFile;

    #[test]
    fn test_runtime_execution() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["safe_tool".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        let req = McpRequest {
            id: "1".into(),
            tool: "safe_tool".into(),
            input: json!({"param": "value"}),
        };

        assert!(runtime.execute(req).is_ok());
        assert_eq!(runtime.sequence(), 1);
    }

    #[test]
    fn test_policy_enforcement() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["safe_tool".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        let req = McpRequest {
            id: "1".into(),
            tool: "dangerous_tool".into(),
            input: json!({}),
        };

        let result = runtime.execute(req);
        assert!(result.is_err());
        assert_eq!(runtime.sequence(), 0); // Sequence unchanged on policy denial
    }

    #[test]
    fn test_crash_and_replay() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();

        {
            // First runtime instance
            let ledger = Ledger::new(path);
            let policy = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
            let mut runtime = Runtime::new(ledger, policy).unwrap();

            runtime
                .execute(McpRequest {
                    id: "1".into(),
                    tool: "tool1".into(),
                    input: json!({}),
                })
                .unwrap();

            runtime
                .execute(McpRequest {
                    id: "2".into(),
                    tool: "tool2".into(),
                    input: json!({}),
                })
                .unwrap();

            assert_eq!(runtime.sequence(), 2);
        } // Runtime dropped (simulating crash)

        {
            // Second runtime instance (after "crash")
            let ledger = Ledger::new(path);
            let policy = AllowListPolicy::new(vec!["tool1".into(), "tool2".into()]);
            let runtime = Runtime::new(ledger, policy).unwrap();

            // Sequence restored from ledger
            assert_eq!(runtime.sequence(), 2);

            // Replay all requests
            let replayed = runtime.replay_all().unwrap();
            assert_eq!(replayed.len(), 2);
            assert_eq!(replayed[0].tool, "tool1");
            assert_eq!(replayed[1].tool, "tool2");
        }
    }

    #[test]
    fn test_context_tracking() {
        let temp_file = NamedTempFile::new().unwrap();
        let ledger = Ledger::new(temp_file.path().to_str().unwrap());
        let policy = AllowListPolicy::new(vec!["tool1".into()]);
        let mut runtime = Runtime::new(ledger, policy).unwrap();

        let context = PolicyContext {
            initiator: "test_user".into(),
            execution_history: vec![],
            metadata: json!({}),
        };
        runtime.set_context(context);

        runtime
            .execute(McpRequest {
                id: "1".into(),
                tool: "tool1".into(),
                input: json!({}),
            })
            .unwrap();

        // Context should track executed tool
        assert_eq!(runtime.context.as_ref().unwrap().execution_history.len(), 1);
        assert_eq!(
            runtime.context.as_ref().unwrap().execution_history[0],
            "tool1"
        );
    }
}
