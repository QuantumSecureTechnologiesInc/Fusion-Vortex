//! Fusion Browser Agent
//!
//! Browser subagent with policy enforcement for navigation
//! and JavaScript execution.

use anyhow::Result;
use fusion_review_policy::BrowserPolicy;
use serde_json::Value;

pub struct BrowserSubagent {
    policy: BrowserPolicy,
    // session: BrowserSession, // TODO: Implement browser session
}

impl BrowserSubagent {
    pub fn new(policy: BrowserPolicy) -> Self {
        Self { policy }
    }

    pub async fn navigate(&mut self, url: &str) -> Result<()> {
        // Check allowlist/denylist
        self.policy_check_url(url)?;

        // TODO: Execute navigation
        Ok(())
    }

    pub async fn execute_javascript(&mut self, script: &str) -> Result<Value> {
        // TODO: Check policy and execute
        Ok(Value::Null)
    }

    fn policy_check_url(&self, url: &str) -> Result<()> {
        // TODO: Implement URL policy checking
        Ok(())
    }
}
