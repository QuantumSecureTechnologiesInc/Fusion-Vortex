//! Agent mode types and execution logic
//!
//! Supports two core modes:
//! - Planning: Deep research, task breakdown, artifact generation
//! - Fast: Direct execution for simple tasks

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentModeType {
    /// Planning mode: deep research, task groups, artifacts
    Planning,
    /// Fast mode: direct execution for simple tasks
    Fast,
}

impl Default for AgentModeType {
    fn default() -> Self {
        Self::Planning
    }
}

impl std::str::FromStr for AgentModeType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "planning" => Ok(Self::Planning),
            "fast" => Ok(Self::Fast),
            _ => anyhow::bail!("Invalid mode: {}. Must be 'planning' or 'fast'", s),
        }
    }
}

impl std::fmt::Display for AgentModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planning => write!(f, "planning"),
            Self::Fast => write!(f, "fast"),
        }
    }
}

/// Agent mode configuration and execution
pub struct AgentMode {
    pub mode_type: AgentModeType,
    pub reasoning_enabled: bool,
}

impl AgentMode {
    pub fn new(mode_type: AgentModeType) -> Self {
        Self {
            mode_type,
            reasoning_enabled: mode_type == AgentModeType::Planning,
        }
    }

    pub fn planning() -> Self {
        Self::new(AgentModeType::Planning)
    }

    pub fn fast() -> Self {
        Self::new(AgentModeType::Fast)
    }

    /// Check if mode requires task group breakdown
    pub fn requires_task_groups(&self) -> bool {
        self.mode_type == AgentModeType::Planning
    }

    /// Check if mode generates artifacts
    pub fn generates_artifacts(&self) -> bool {
        self.mode_type == AgentModeType::Planning
    }

    /// Check if mode performs deep research
    pub fn performs_research(&self) -> bool {
        self.mode_type == AgentModeType::Planning
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_from_str() {
        assert_eq!(
            "planning".parse::<AgentModeType>().unwrap(),
            AgentModeType::Planning
        );
        assert_eq!(
            "fast".parse::<AgentModeType>().unwrap(),
            AgentModeType::Fast
        );
        assert_eq!(
            "PLANNING".parse::<AgentModeType>().unwrap(),
            AgentModeType::Planning
        );
    }

    #[test]
    fn test_mode_display() {
        assert_eq!(AgentModeType::Planning.to_string(), "planning");
        assert_eq!(AgentModeType::Fast.to_string(), "fast");
    }

    #[test]
    fn test_planning_mode_features() {
        let mode = AgentMode::planning();
        assert!(mode.requires_task_groups());
        assert!(mode.generates_artifacts());
        assert!(mode.performs_research());
    }

    #[test]
    fn test_fast_mode_features() {
        let mode = AgentMode::fast();
        assert!(!mode.requires_task_groups());
        assert!(!mode.generates_artifacts());
        assert!(!mode.performs_research());
    }
}
