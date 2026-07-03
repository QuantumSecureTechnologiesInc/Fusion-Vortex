use fusion_std::error::{StdError, StdResult};
use regex::Regex;
use std::collections::HashMap;

pub enum ViolationType {
    PiiLeak,
    HarmfulContent,
    Hallucination,
    RateLimitExceeded,
    ProhibitedKeyword,
}

pub struct SafetyMonitor {
    pii_patterns: Vec<Regex>,
    harmful_keywords: Vec<String>,
    violations: Vec<ViolationRecord>,
}

#[allow(dead_code)]
struct ViolationRecord {
    violation_type: String,
    timestamp: std::time::SystemTime,
    metadata: HashMap<String, String>,
}

impl SafetyMonitor {
    pub fn new() -> Self {
        // Initialize PII detection patterns
        let pii_patterns = vec![
            // Email addresses
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
            // Phone numbers (US format)
            Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
            // Credit card numbers (simplified)
            Regex::new(r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b").unwrap(),
            // Social Security Numbers
            Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),
        ];

        // Harmful content keywords
        let harmful_keywords = vec![
            "violence".to_string(),
            "exploit".to_string(),
            "malware".to_string(),
            "phishing".to_string(),
        ];

        Self {
            pii_patterns,
            harmful_keywords,
            violations: Vec::new(),
        }
    }

    pub fn record_violation(
        &mut self,
        violation: ViolationType,
        metadata: HashMap<String, String>,
    ) {
        let violation_type = match violation {
            ViolationType::PiiLeak => "PII_LEAK",
            ViolationType::HarmfulContent => "HARMFUL_CONTENT",
            ViolationType::Hallucination => "HALLUCINATION",
            ViolationType::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            ViolationType::ProhibitedKeyword => "PROHIBITED_KEYWORD",
        };

        println!("[SAFETY VIOLATION] {}: {:?}", violation_type, metadata);

        let record = ViolationRecord {
            violation_type: violation_type.to_string(),
            timestamp: std::time::SystemTime::now(),
            metadata,
        };

        self.violations.push(record);
    }

    pub async fn check_output(&mut self, user_id: &str, output: &str) -> StdResult<()> {
        // 1. Check for PII leaks
        for pattern in &self.pii_patterns {
            if pattern.is_match(output) {
                let mut metadata = HashMap::new();
                metadata.insert("user_id".to_string(), user_id.to_string());
                metadata.insert("pattern".to_string(), pattern.as_str().to_string());
                self.record_violation(ViolationType::PiiLeak, metadata);
                return Err(StdError::PermissionDenied("PII detected in output".into()));
            }
        }

        // 2. Check for harmful keywords
        let output_lower = output.to_lowercase();
        let harmful_keywords = self.harmful_keywords.clone();
        for keyword in &harmful_keywords {
            if output_lower.contains(keyword) {
                let mut metadata = HashMap::new();
                metadata.insert("user_id".to_string(), user_id.to_string());
                metadata.insert("keyword".to_string(), keyword.clone());
                self.record_violation(ViolationType::ProhibitedKeyword, metadata);
                return Err(StdError::PermissionDenied(format!(
                    "Prohibited keyword detected: {}",
                    keyword
                )));
            }
        }

        // 3. Check for repetitive patterns (hallucination indicator)
        if Self::detect_repetition(output) {
            let mut metadata = HashMap::new();
            metadata.insert("user_id".to_string(), user_id.to_string());
            self.record_violation(ViolationType::Hallucination, metadata);
            // Don't block, just log
        }

        Ok(())
    }

    fn detect_repetition(text: &str) -> bool {
        // Simple repetition detection: check for repeated 3+ word sequences
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() < 6 {
            return false;
        }

        for i in 0..words.len() - 5 {
            let sequence = &words[i..i + 3];
            let rest = &words[i + 3..];

            // Check if this 3-word sequence appears again
            for j in 0..rest.len() - 2 {
                if rest[j..j + 3] == *sequence {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_violation_count(&self) -> usize {
        self.violations.len()
    }

    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pii_detection() {
        let mut monitor = SafetyMonitor::new();
        let result = monitor
            .check_output("user123", "Contact me at test@example.com")
            .await;
        assert!(result.is_err());
        assert_eq!(monitor.get_violation_count(), 1);
    }

    #[tokio::test]
    async fn test_harmful_keyword_detection() {
        let mut monitor = SafetyMonitor::new();
        let result = monitor
            .check_output("user123", "This contains malware instructions")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_safe_output() {
        let mut monitor = SafetyMonitor::new();
        let result = monitor
            .check_output("user123", "This is a safe message")
            .await;
        assert!(result.is_ok());
    }
}
