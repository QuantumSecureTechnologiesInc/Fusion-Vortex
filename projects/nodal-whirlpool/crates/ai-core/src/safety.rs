use serde::{Deserialize, Serialize};

/// Safety levels for generated content
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SafetyLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

/// Safety report for generated content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyReport {
    pub level: SafetyLevel,
    pub issues: Vec<SafetyIssue>,
    pub requires_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyIssue {
    pub kind: SafetyIssueKind,
    pub description: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SafetyIssueKind {
    PII,             // Personally Identifiable Information
    Secret,          // API keys, passwords, tokens
    LicenseConflict, // License incompatibility
    SecurityRisk,    // Potential security vulnerability
    MaliciousCode,   // Detected malicious patterns
}

/// Safety engine for validating AI-generated content
pub struct SafetyEngine {
    policies: Vec<SafetyPolicy>,
}

#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    pub name: String,
    pub enabled: bool,
}

impl SafetyEngine {
    pub fn new() -> Self {
        Self {
            policies: Self::default_policies(),
        }
    }

    fn default_policies() -> Vec<SafetyPolicy> {
        vec![
            SafetyPolicy {
                name: "pii_detection".to_string(),
                enabled: true,
            },
            SafetyPolicy {
                name: "secret_detection".to_string(),
                enabled: true,
            },
            SafetyPolicy {
                name: "license_check".to_string(),
                enabled: true,
            },
        ]
    }

    /// Verify generated content for safety issues
    pub fn verify(&self, content: &str) -> SafetyReport {
        let mut issues = Vec::new();

        // Check for common patterns
        if self.check_secrets(content) {
            issues.push(SafetyIssue {
                kind: SafetyIssueKind::Secret,
                description: "Potential secret detected".to_string(),
                line: None,
                column: None,
            });
        }

        if self.check_pii(content) {
            issues.push(SafetyIssue {
                kind: SafetyIssueKind::PII,
                description: "Potential PII detected".to_string(),
                line: None,
                column: None,
            });
        }

        let level = Self::calculate_level(&issues);
        let requires_review = level >= SafetyLevel::Medium;

        SafetyReport {
            level,
            issues,
            requires_review,
        }
    }

    fn check_secrets(&self, content: &str) -> bool {
        // Simple pattern matching for common secret formats
        let patterns = [
            "api_key",
            "secret_key",
            "password",
            "token",
            "sk-",  // OpenAI style
            "ghp_", // GitHub token
        ];

        patterns.iter().any(|p| content.to_lowercase().contains(p))
    }

    fn check_pii(&self, content: &str) -> bool {
        // Simple pattern matching for PII
        let patterns = ["@gmail.com", "@outlook.com", "ssn", "social security"];

        patterns.iter().any(|p| content.to_lowercase().contains(p))
    }

    fn calculate_level(issues: &[SafetyIssue]) -> SafetyLevel {
        if issues.is_empty() {
            return SafetyLevel::Safe;
        }

        let has_critical = issues
            .iter()
            .any(|i| matches!(i.kind, SafetyIssueKind::MaliciousCode));
        if has_critical {
            return SafetyLevel::Critical;
        }

        let has_high = issues.iter().any(|i| {
            matches!(
                i.kind,
                SafetyIssueKind::Secret | SafetyIssueKind::SecurityRisk
            )
        });
        if has_high {
            return SafetyLevel::High;
        }

        if issues.len() > 2 {
            SafetyLevel::Medium
        } else {
            SafetyLevel::Low
        }
    }
}

impl Default for SafetyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_content() {
        let engine = SafetyEngine::new();
        let report = engine.verify("fn main() { println!(\"Hello\"); }");
        assert_eq!(report.level, SafetyLevel::Safe);
        assert!(!report.requires_review);
    }

    #[test]
    fn test_detect_secret() {
        let engine = SafetyEngine::new();
        let report = engine.verify("let api_key = \"sk-1234567890\";");
        assert!(report.level >= SafetyLevel::Low);
        assert!(report
            .issues
            .iter()
            .any(|i| i.kind == SafetyIssueKind::Secret));
    }
}
