use serde::{Deserialize, Serialize};

/// Policy decision outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyDecision {
    /// Execution is allowed
    Allow,
    /// Execution is denied with reason
    Deny(String),
}

impl PolicyDecision {
    /// Check if the decision allows execution
    pub fn is_allow(&self) -> bool {
        matches!(self, PolicyDecision::Allow)
    }

    /// Check if the decision denies execution
    pub fn is_deny(&self) -> bool {
        matches!(self, PolicyDecision::Deny(_))
    }

    /// Get the denial reason if denied
    pub fn denial_reason(&self) -> Option<&str> {
        match self {
            PolicyDecision::Deny(reason) => Some(reason),
            PolicyDecision::Allow => None,
        }
    }
}

/// Policy trait for evaluating tool execution permissions
///
/// **Zero implicit permissions. Ever.**
///
/// Policies are evaluated BEFORE execution. If a policy says no, nothing runs.
pub trait Policy {
    /// Evaluate whether the given tool is permitted to execute
    ///
    /// # Arguments
    /// * `tool` - The name of the tool requesting execution
    /// * `context` - Optional execution context for more sophisticated policies
    ///
    /// # Returns
    /// `PolicyDecision::Allow` if permitted, `PolicyDecision::Deny(reason)` otherwise
    fn evaluate(&self, tool: &str, context: Option<&PolicyContext>) -> PolicyDecision;

    /// Evaluate a simple tool permission without context
    fn evaluate_simple(&self, tool: &str) -> PolicyDecision {
        self.evaluate(tool, None)
    }
}

/// Execution context for policy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyContext {
    /// User or agent initiating the request
    pub initiator: String,
    /// Previous tools executed in this session
    pub execution_history: Vec<String>,
    /// Additional metadata
    pub metadata: serde_json::Value,
}

/// Allow-list based policy implementation
///
/// Only tools explicitly listed are allowed. All others are denied.
#[derive(Debug, Clone)]
pub struct AllowListPolicy {
    allowed: Vec<String>,
}

impl AllowListPolicy {
    /// Create a new allow-list policy
    ///
    /// # Example
    /// ```
    /// use fusion_policy::{AllowListPolicy, Policy, PolicyDecision};
    ///
    /// let policy = AllowListPolicy::new(vec!["safe_tool".into(), "read_file".into()]);
    /// assert!(policy.evaluate_simple("safe_tool").is_allow());
    /// assert!(policy.evaluate_simple("dangerous_tool").is_deny());
    /// ```
    pub fn new(allowed: Vec<String>) -> Self {
        Self { allowed }
    }

    /// Add a tool to the allow list
    pub fn allow(&mut self, tool: impl Into<String>) {
        self.allowed.push(tool.into());
    }

    /// Remove a tool from the allow list
    pub fn deny(&mut self, tool: &str) {
        self.allowed.retain(|t| t != tool);
    }

    /// Check if a tool is in the allow list
    pub fn is_allowed(&self, tool: &str) -> bool {
        self.allowed.contains(&tool.to_string())
    }
}

impl Policy for AllowListPolicy {
    fn evaluate(&self, tool: &str, _context: Option<&PolicyContext>) -> PolicyDecision {
        if self.allowed.contains(&tool.to_string()) {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Deny(format!(
                "Tool '{}' not permitted by allow-list policy",
                tool
            ))
        }
    }
}

/// Deny-list based policy implementation
///
/// All tools are allowed except those explicitly denied.
#[derive(Debug, Clone)]
pub struct DenyListPolicy {
    denied: Vec<String>,
}

impl DenyListPolicy {
    /// Create a new deny-list policy
    ///
    /// # Example
    /// ```
    /// use fusion_policy::{DenyListPolicy, Policy, PolicyDecision};
    ///
    /// let policy = DenyListPolicy::new(vec!["dangerous_tool".into()]);
    /// assert!(policy.evaluate_simple("safe_tool").is_allow());
    /// assert!(policy.evaluate_simple("dangerous_tool").is_deny());
    /// ```
    pub fn new(denied: Vec<String>) -> Self {
        Self { denied }
    }

    /// Add a tool to the deny list
    pub fn deny(&mut self, tool: impl Into<String>) {
        self.denied.push(tool.into());
    }

    /// Remove a tool from the deny list
    pub fn allow(&mut self, tool: &str) {
        self.denied.retain(|t| t != tool);
    }
}

impl Policy for DenyListPolicy {
    fn evaluate(&self, tool: &str, _context: Option<&PolicyContext>) -> PolicyDecision {
        if self.denied.contains(&tool.to_string()) {
            PolicyDecision::Deny(format!(
                "Tool '{}' explicitly denied by deny-list policy",
                tool
            ))
        } else {
            PolicyDecision::Allow
        }
    }
}

/// Composite policy that combines multiple policies with AND logic
///
/// All constituent policies must allow for the composite to allow.
pub struct CompositeAndPolicy {
    policies: Vec<Box<dyn Policy>>,
}

impl CompositeAndPolicy {
    /// Create a new composite AND policy
    pub fn new() -> Self {
        Self { policies: vec![] }
    }

    /// Add a policy to the composite
    pub fn add_policy(&mut self, policy: Box<dyn Policy>) {
        self.policies.push(policy);
    }
}

impl Default for CompositeAndPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl Policy for CompositeAndPolicy {
    fn evaluate(&self, tool: &str, context: Option<&PolicyContext>) -> PolicyDecision {
        for policy in &self.policies {
            let decision = policy.evaluate(tool, context);
            if decision.is_deny() {
                return decision;
            }
        }
        PolicyDecision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_list_policy() {
        let policy = AllowListPolicy::new(vec!["safe_tool".into(), "read_file".into()]);

        assert_eq!(policy.evaluate_simple("safe_tool"), PolicyDecision::Allow);
        assert_eq!(policy.evaluate_simple("read_file"), PolicyDecision::Allow);

        let deny_result = policy.evaluate_simple("dangerous_tool");
        assert!(deny_result.is_deny());
        assert!(deny_result
            .denial_reason()
            .unwrap()
            .contains("dangerous_tool"));
    }

    #[test]
    fn test_deny_list_policy() {
        let policy = DenyListPolicy::new(vec!["dangerous_tool".into()]);

        assert_eq!(policy.evaluate_simple("safe_tool"), PolicyDecision::Allow);

        let deny_result = policy.evaluate_simple("dangerous_tool");
        assert!(deny_result.is_deny());
    }

    #[test]
    fn test_composite_and_policy() {
        let mut composite = CompositeAndPolicy::new();
        composite.add_policy(Box::new(AllowListPolicy::new(vec![
            "tool1".into(),
            "tool2".into(),
        ])));
        composite.add_policy(Box::new(DenyListPolicy::new(vec!["tool2".into()])));

        // tool1 is allowed by allow-list and not denied by deny-list
        assert_eq!(composite.evaluate_simple("tool1"), PolicyDecision::Allow);

        // tool2 is allowed by allow-list but denied by deny-list
        assert!(composite.evaluate_simple("tool2").is_deny());

        // tool3 is not in allow-list
        assert!(composite.evaluate_simple("tool3").is_deny());
    }

    #[test]
    fn test_policy_decision_helpers() {
        let allow = PolicyDecision::Allow;
        assert!(allow.is_allow());
        assert!(!allow.is_deny());
        assert_eq!(allow.denial_reason(), None);

        let deny = PolicyDecision::Deny("test reason".into());
        assert!(!deny.is_allow());
        assert!(deny.is_deny());
        assert_eq!(deny.denial_reason(), Some("test reason"));
    }
}
