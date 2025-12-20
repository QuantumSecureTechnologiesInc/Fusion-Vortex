use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Compliance error
#[derive(Error, Debug, Clone)]
pub enum ComplianceError {
    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Audit failed: {0}")]
    AuditFailed(String),

    #[error("Report generation failed: {0}")]
    ReportFailed(String),
}

/// Regulatory framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatoryFramework {
    GDPR,
    SOC2,
    HIPAA,
    Custom,
}

/// Data classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub id: String,
    pub framework: RegulatoryFramework,
    pub rule: String,
    pub description: String,
    pub severity: ViolationSeverity,
    pub detected_at: DateTime<Utc>,
}

/// Violation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Data processing activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataActivity {
    pub id: String,
    pub data_type: String,
    pub purpose: String,
    pub classification: DataClassification,
    pub retention_days: u32,
    pub created_at: DateTime<Utc>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub framework: RegulatoryFramework,
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_activities: usize,
    pub violations: Vec<Violation>,
    pub compliance_score: f64,
}

/// Compliance engine
pub struct ComplianceEngine {
    framework: RegulatoryFramework,
    violations: Vec<Violation>,
    activities: HashMap<String, DataActivity>,
    retention_policies: HashMap<String, u32>,
}

impl ComplianceEngine {
    /// Create a new compliance engine
    pub fn new(framework: RegulatoryFramework) -> Self {
        let mut engine = Self {
            framework,
            violations: vec![],
            activities: HashMap::new(),
            retention_policies: HashMap::new(),
        };

        engine.initialize_defaults();
        engine
    }

    fn initialize_defaults(&mut self) {
        // Set default retention policies based on framework
        match self.framework {
            RegulatoryFramework::GDPR => {
                self.retention_policies
                    .insert("personal_data".to_string(), 365);
                self.retention_policies
                    .insert("consent_records".to_string(), 730);
            }
            RegulatoryFramework::HIPAA => {
                self.retention_policies
                    .insert("health_records".to_string(), 2190); // 6 years
                self.retention_policies
                    .insert("audit_logs".to_string(), 2190);
            }
            RegulatoryFramework::SOC2 => {
                self.retention_policies
                    .insert("security_logs".to_string(), 365);
                self.retention_policies
                    .insert("access_records".to_string(), 365);
            }
            _ => {}
        }
    }

    /// Register a data processing activity
    pub fn register_activity(&mut self, activity: DataActivity) -> Result<(), ComplianceError> {
        // Validate based on framework
        match self.framework {
            RegulatoryFramework::GDPR => {
                self.validate_gdpr_activity(&activity)?;
            }
            RegulatoryFramework::HIPAA => {
                self.validate_hipaa_activity(&activity)?;
            }
            RegulatoryFramework::SOC2 => {
                self.validate_soc2_activity(&activity)?;
            }
            _ => {}
        }

        self.activities.insert(activity.id.clone(), activity);
        Ok(())
    }

    fn validate_gdpr_activity(&mut self, activity: &DataActivity) -> Result<(), ComplianceError> {
        // GDPR requires purpose limitation
        if activity.purpose.is_empty() {
            let violation = Violation {
                id: format!("gdpr-{}", Utc::now().timestamp()),
                framework: RegulatoryFramework::GDPR,
                rule: "Purpose Limitation".to_string(),
                description: format!("Activity {} missing purpose", activity.id),
                severity: ViolationSeverity::High,
                detected_at: Utc::now(),
            };
            self.violations.push(violation.clone());
            return Err(ComplianceError::PolicyViolation(
                "GDPR requires explicit purpose for data processing".to_string(),
            ));
        }

        // Check retention period
        if let Some(&max_retention) = self.retention_policies.get(&activity.data_type) {
            if activity.retention_days > max_retention {
                let violation = Violation {
                    id: format!("gdpr-{}", Utc::now().timestamp()),
                    framework: RegulatoryFramework::GDPR,
                    rule: "Storage Limitation".to_string(),
                    description: format!(
                        "Activity {} exceeds maximum retention period ({} > {})",
                        activity.id, activity.retention_days, max_retention
                    ),
                    severity: ViolationSeverity::Medium,
                    detected_at: Utc::now(),
                };
                self.violations.push(violation);
            }
        }

        Ok(())
    }

    fn validate_hipaa_activity(&mut self, activity: &DataActivity) -> Result<(), ComplianceError> {
        // HIPAA requires restricted classification for health data
        if activity.data_type.contains("health")
            && activity.classification != DataClassification::Restricted
        {
            let violation = Violation {
                id: format!("hipaa-{}", Utc::now().timestamp()),
                framework: RegulatoryFramework::HIPAA,
                rule: "PHI Protection".to_string(),
                description: format!("Health data must be classified as Restricted"),
                severity: ViolationSeverity::Critical,
                detected_at: Utc::now(),
            };
            self.violations.push(violation.clone());
            return Err(ComplianceError::PolicyViolation(
                "HIPAA requires health data to be classified as Restricted".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_soc2_activity(&mut self, activity: &DataActivity) -> Result<(), ComplianceError> {
        // SOC2 requires appropriate classification
        if activity.classification == DataClassification::Public
            && (activity.data_type.contains("security") || activity.data_type.contains("access"))
        {
            let violation = Violation {
                id: format!("soc2-{}", Utc::now().timestamp()),
                framework: RegulatoryFramework::SOC2,
                rule: "Security Controls".to_string(),
                description: format!("Security data cannot be classified as Public"),
                severity: ViolationSeverity::High,
                detected_at: Utc::now(),
            };
            self.violations.push(violation.clone());
            return Err(ComplianceError::PolicyViolation(
                "SOC2 requires proper classification of security data".to_string(),
            ));
        }

        Ok(())
    }

    /// Generate a compliance report
    pub fn generate_report(
        &self,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> ComplianceReport {
        let total = self.activities.len();
        let violation_count = self.violations.len();

        // Calculate compliance score (percentage of activities without violations)
        let compliance_score = if total > 0 {
            ((total - violation_count.min(total)) as f64 / total as f64) * 100.0
        } else {
            100.0
        };

        ComplianceReport {
            framework: self.framework,
            generated_at: Utc::now(),
            period_start,
            period_end,
            total_activities: total,
            violations: self.violations.clone(),
            compliance_score,
        }
    }

    /// Get all violations
    pub fn get_violations(&self) -> &[Violation] {
        &self.violations
    }

    /// Get violations by severity
    pub fn get_violations_by_severity(&self, severity: ViolationSeverity) -> Vec<&Violation> {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .collect()
    }

    /// Clear violations (after remediation)
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }

    /// Get activity count
    pub fn activity_count(&self) -> usize {
        self.activities.len()
    }

    /// Set retention policy for a data type
    pub fn set_retention_policy(&mut self, data_type: String, retention_days: u32) {
        self.retention_policies.insert(data_type, retention_days);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_engine_creation() {
        let engine = ComplianceEngine::new(RegulatoryFramework::GDPR);
        assert_eq!(engine.framework, RegulatoryFramework::GDPR);
        assert!(engine.retention_policies.contains_key("personal_data"));
    }

    #[test]
    fn test_gdpr_purpose_validation() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::GDPR);

        let activity = DataActivity {
            id: "act1".to_string(),
            data_type: "personal_data".to_string(),
            purpose: String::new(), // Empty purpose - should violate GDPR
            classification: DataClassification::Confidential,
            retention_days: 30,
            created_at: Utc::now(),
        };

        let result = engine.register_activity(activity);
        assert!(result.is_err());
        assert_eq!(engine.get_violations().len(), 1);
    }

    #[test]
    fn test_hipaa_classification_validation() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::HIPAA);

        let activity = DataActivity {
            id: "act1".to_string(),
            data_type: "health_records".to_string(),
            purpose: "treatment".to_string(),
            classification: DataClassification::Internal, // Should be Restricted
            retention_days: 2000,
            created_at: Utc::now(),
        };

        let result = engine.register_activity(activity);
        assert!(result.is_err());
        assert_eq!(engine.get_violations().len(), 1);
    }

    #[test]
    fn test_valid_activity_registration() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::GDPR);

        let activity = DataActivity {
            id: "act1".to_string(),
            data_type: "personal_data".to_string(),
            purpose: "user authentication".to_string(),
            classification: DataClassification::Confidential,
            retention_days: 365,
            created_at: Utc::now(),
        };

        let result = engine.register_activity(activity);
        assert!(result.is_ok());
        assert_eq!(engine.activity_count(), 1);
    }

    #[test]
    fn test_retention_policy_violation() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::GDPR);

        let activity = DataActivity {
            id: "act1".to_string(),
            data_type: "personal_data".to_string(),
            purpose: "analytics".to_string(),
            classification: DataClassification::Confidential,
            retention_days: 1000, // Exceeds default 365 days
            created_at: Utc::now(),
        };

        engine.register_activity(activity).ok();
        assert!(!engine.get_violations().is_empty());
    }

    #[test]
    fn test_compliance_report_generation() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::SOC2);

        // Add valid activities
        for i in 0..5 {
            let activity = DataActivity {
                id: format!("act{}", i),
                data_type: "logs".to_string(),
                purpose: "monitoring".to_string(),
                classification: DataClassification::Internal,
                retention_days: 365,
                created_at: Utc::now(),
            };
            engine.register_activity(activity).ok();
        }

        let report = engine.generate_report(Utc::now(), Utc::now());
        assert_eq!(report.total_activities, 5);
        assert_eq!(report.framework, RegulatoryFramework::SOC2);
    }

    #[test]
    fn test_violations_by_severity() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::HIPAA);

        // Trigger a critical violation
        let activity = DataActivity {
            id: "act1".to_string(),
            data_type: "health_records".to_string(),
            purpose: "treatment".to_string(),
            classification: DataClassification::Public, // Critical violation
            retention_days: 100,
            created_at: Utc::now(),
        };

        engine.register_activity(activity).ok();

        let critical = engine.get_violations_by_severity(ViolationSeverity::Critical);
        assert_eq!(critical.len(), 1);
    }

    #[test]
    fn test_clear_violations() {
        let mut engine = ComplianceEngine::new(RegulatoryFramework::GDPR);

        let activity = DataActivity {
            id: "act1".to_string(),
            data_type: "personal_data".to_string(),
            purpose: String::new(),
            classification: DataClassification::Confidential,
            retention_days: 30,
            created_at: Utc::now(),
        };

        engine.register_activity(activity).ok();
        assert!(!engine.get_violations().is_empty());

        engine.clear_violations();
        assert!(engine.get_violations().is_empty());
    }
}
