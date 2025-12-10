use serde::{Deserialize, Serialize};

/// Policy manager for AI operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyManager {
    pub policies: Vec<Policy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub name: String,
    pub enabled: bool,
    pub settings: serde_json::Value,
}

impl PolicyManager {
    pub fn new() -> Self {
        Self {
            policies: Self::default_policies(),
        }
    }

    fn default_policies() -> Vec<Policy> {
        vec![
            Policy {
                name: "max_token_limit".to_string(),
                enabled: true,
                settings: serde_json::json!({ "limit": 4096 }),
            },
            Policy {
                name: "require_human_review".to_string(),
                enabled: true,
                settings: serde_json::json!({ "threshold": "medium" }),
            },
        ]
    }

    pub fn check_policy(&self, policy_name: &str) -> bool {
        self.policies
            .iter()
            .any(|p| p.name == policy_name && p.enabled)
    }
}

impl Default for PolicyManager {
    fn default() -> Self {
        Self::new()
    }
}
