use anyhow::Result;
use fusion_policy::Capability;

/// Safety constraint enforcement for agent operations
pub struct SafetyEnforcer;

impl SafetyEnforcer {
    /// Verify an agent cannot escalate capabilities
    pub fn check_capability_escalation(
        requested: &[Capability],
        allowed: &[Capability],
    ) -> Result<()> {
        for cap in requested {
            if !allowed.contains(cap) {
                anyhow::bail!(
                    "Agent attempted capability escalation: {:?} not in allowed set",
                    cap
                );
            }
        }
        Ok(())
    }

    /// Verify tool is in the declared graph
    pub fn check_tool_declared(tool_name: &str, declared_tools: &[String]) -> Result<()> {
        if !declared_tools.contains(&tool_name.to_string()) {
            anyhow::bail!("Agent attempted to call undeclared tool: {}", tool_name);
        }
        Ok(())
    }

    /// Agents cannot install extensions
    pub fn block_extension_installation(_extension_id: &str) -> Result<()> {
        anyhow::bail!("Agents are not permitted to install extensions")
    }

    /// Agents cannot bypass compatibility levels
    // TODO: Re-enable when vscode-runtime is properly integrated
    /*
    pub fn enforce_compatibility(
        _required: &fusion_vscode_runtime::compat::CompatibilityLevel,
        _current: &fusion_vscode_runtime::compat::CompatibilityLevel,
    ) -> Result<()> {
        // Placeholder for compatibility enforcement logic
        Ok(())
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_escalation_blocked() {
        let requested = vec![Capability::FilesystemWrite, Capability::NetworkOutbound];
        let allowed = vec![Capability::FilesystemRead];

        let result = SafetyEnforcer::check_capability_escalation(&requested, &allowed);
        assert!(result.is_err());
    }

    #[test]
    fn test_undeclared_tool_blocked() {
        let declared = vec!["tool.a".to_string(), "tool.b".to_string()];

        let result = SafetyEnforcer::check_tool_declared("tool.c", &declared);
        assert!(result.is_err());
    }

    #[test]
    fn test_extension_installation_blocked() {
        let result = SafetyEnforcer::block_extension_installation("malicious.extension");
        assert!(result.is_err());
    }
}
