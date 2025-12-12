//! Code excellence enforcement and quality metrics

use crate::{AgenticError, Result};
use serde::{Deserialize, Serialize};

/// Quality metrics for code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Overall quality score (0.0 - 100.0)
    pub overall_score: f64,

    /// Readability score (0.0 - 100.0)
    pub readability: f64,

    /// Maintainability score (0.0 - 100.0)
    pub maintainability: f64,

    /// Performance score (0.0 - 100.0)
    pub performance: f64,

    /// Security score (0.0 - 100.0)
    pub security: f64,

    /// Test coverage (0.0 - 100.0)
    pub test_coverage: f64,

    /// Documentation score (0.0 - 100.0)
    pub documentation: f64,

    /// Issues found
    pub issues: Vec<QualityIssue>,

    /// Recommendations
    pub recommendations: Vec<String>,
}

impl QualityMetrics {
    pub fn new() -> Self {
        Self {
            overall_score: 0.0,
            readability: 0.0,
            maintainability: 0.0,
            performance: 0.0,
            security: 0.0,
            test_coverage: 0.0,
            documentation: 0.0,
            issues: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn calculate_overall(&mut self) {
        self.overall_score = (self.readability * 0.2
            + self.maintainability * 0.25
            + self.performance * 0.15
            + self.security * 0.25
            + self.test_coverage * 0.1
            + self.documentation * 0.05);
    }

    pub fn is_excellent(&self) -> bool {
        self.overall_score >= 90.0
    }

    pub fn is_good(&self) -> bool {
        self.overall_score >= 75.0
    }

    pub fn needs_improvement(&self) -> bool {
        self.overall_score < 60.0
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// A code quality issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Issue severity
    pub severity: IssueSeverity,

    /// Issue category
    pub category: IssueCategory,

    /// Issue description
    pub description: String,

    /// Line number (if applicable)
    pub line: Option<usize>,

    /// Suggested fix
    pub fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueCategory {
    Security,
    Performance,
    Readability,
    Maintainability,
    BestPractices,
    Documentation,
    Testing,
}

/// Code standards to enforce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStandard {
    /// Standard name
    pub name: String,

    /// Standard rules
    pub rules: Vec<StandardRule>,

    /// Minimum acceptable score
    pub min_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardRule {
    /// Rule name
    pub name: String,

    /// Rule description
    pub description: String,

    /// Rule weight (0.0 - 1.0)
    pub weight: f64,

    /// Rule category
    pub category: IssueCategory,
}

/// Excellence enforcer
pub struct ExcellenceEnforcer {
    /// Code standards
    standards: Vec<CodeStandard>,

    /// Minimum overall score
    min_overall_score: f64,

    /// Strict mode
    strict_mode: bool,
}

impl ExcellenceEnforcer {
    pub fn new() -> Self {
        let mut enforcer = Self {
            standards: Vec::new(),
            min_overall_score: 80.0,
            strict_mode: true,
        };

        enforcer.load_default_standards();
        enforcer
    }

    fn load_default_standards(&mut self) {
        // Readability standard
        self.standards.push(CodeStandard {
            name: "Readability".to_string(),
            min_score: 75.0,
            rules: vec![
                StandardRule {
                    name: "Clear naming".to_string(),
                    description: "Use descriptive, meaningful names".to_string(),
                    weight: 0.3,
                    category: IssueCategory::Readability,
                },
                StandardRule {
                    name: "Comment quality".to_string(),
                    description: "Provide clear, concise comments".to_string(),
                    weight: 0.2,
                    category: IssueCategory::Documentation,
                },
                StandardRule {
                    name: "Code formatting".to_string(),
                    description: "Consistent formatting and style".to_string(),
                    weight: 0.2,
                    category: IssueCategory::Readability,
                },
            ],
        });

        // Security standard
        self.standards.push(CodeStandard {
            name: "Security".to_string(),
            min_score: 90.0,
            rules: vec![
                StandardRule {
                    name: "Input validation".to_string(),
                    description: "Validate all external inputs".to_string(),
                    weight: 0.4,
                    category: IssueCategory::Security,
                },
                StandardRule {
                    name: "Error handling".to_string(),
                    description: "Proper error handling without information leakage".to_string(),
                    weight: 0.3,
                    category: IssueCategory::Security,
                },
                StandardRule {
                    name: "Dependency security".to_string(),
                    description: "Use secure, up-to-date dependencies".to_string(),
                    weight: 0.3,
                    category: IssueCategory::Security,
                },
            ],
        });

        // Performance standard
        self.standards.push(CodeStandard {
            name: "Performance".to_string(),
            min_score: 70.0,
            rules: vec![
                StandardRule {
                    name: "Algorithmic efficiency".to_string(),
                    description: "Use efficient algorithms and data structures".to_string(),
                    weight: 0.4,
                    category: IssueCategory::Performance,
                },
                StandardRule {
                    name: "Memory management".to_string(),
                    description: "Efficient memory usage and cleanup".to_string(),
                    weight: 0.3,
                    category: IssueCategory::Performance,
                },
            ],
        });
    }

    /// Analyse code and generate quality metrics
    pub fn analyse(&self, code: &str) -> Result<QualityMetrics> {
        let mut metrics = QualityMetrics::new();

        // Readability analysis
        metrics.readability = self.analyse_readability(code);

        // Maintainability analysis
        metrics.maintainability = self.analyse_maintainability(code);

        // Performance analysis
        metrics.performance = self.analyse_performance(code);

        // Security analysis
        metrics.security = self.analyse_security(code);

        // Documentation analysis
        metrics.documentation = self.analyse_documentation(code);

        // Test coverage (simplified)
        metrics.test_coverage = self.analyse_test_coverage(code);

        // Calculate overall score
        metrics.calculate_overall();

        // Generate recommendations
        metrics.recommendations = self.generate_recommendations(&metrics);

        Ok(metrics)
    }

    fn analyse_readability(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check line length
        for line in code.lines() {
            if line.len() > 100 {
                score -= 2.0;
            }
        }

        // Check for comments
        let comment_lines = code.lines().filter(|l| l.trim().starts_with("//")).count();
        let total_lines = code.lines().count();
        if total_lines > 0 {
            let comment_ratio = comment_lines as f64 / total_lines as f64;
            if comment_ratio < 0.1 {
                score -= 10.0;
            }
        }

        score.max(0.0)
    }

    fn analyse_maintainability(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;
        let lines = code.lines().count();

        // Penalise very long files
        if lines > 500 {
            score -= 20.0;
        } else if lines > 300 {
            score -= 10.0;
        }

        // Check for modularity indicators
        if !code.contains("fn ") && lines > 20 {
            score -= 15.0;
        }

        score.max(0.0)
    }

    fn analyse_performance(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check for common performance anti-patterns
        if code.contains("clone()") && code.matches("clone()").count() > 5 {
            score -= 10.0;
        }

        // Check for inefficient patterns
        if code.contains("collect()") && code.contains("iter()") {
            // Potential iterator chain - generally good
            score += 5.0;
        }

        score.min(100.0).max(0.0)
    }

    fn analyse_security(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;
        let mut _issues = Vec::new();

        // Check for unsafe code
        if code.contains("unsafe") {
            score -= 20.0;
            _issues.push("Contains unsafe code");
        }

        // Check for unwrap()
        let unwrap_count = code.matches("unwrap()").count();
        if unwrap_count > 0 {
            score -= (unwrap_count as f64 * 5.0);
            _issues.push("Uses unwrap() which can panic");
        }

        // Check for expect()
        let expect_count = code.matches("expect(").count();
        if expect_count > 3 {
            score -= 5.0;
        }

        score.max(0.0)
    }

    fn analyse_documentation(&self, code: &str) -> f64 {
        let mut score: f64 = 100.0;

        // Check for doc comments
        let doc_comments = code.lines().filter(|l| l.trim().starts_with("///")).count();
        let functions = code.matches("fn ").count();

        if functions > 0 {
            let doc_ratio = doc_comments as f64 / functions as f64;
            if doc_ratio < 0.5 {
                score -= 30.0;
            }
        }

        // Check for module-level docs
        if !code.contains("//!") && code.len() > 100 {
            score -= 20.0;
        }

        score.max(0.0)
    }

    fn analyse_test_coverage(&self, code: &str) -> f64 {
        // Check for test module
        if code.contains("#[cfg(test)]") {
            75.0
        } else if code.contains("#[test]") {
            60.0
        } else {
            0.0
        }
    }

    fn generate_recommendations(&self, metrics: &QualityMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.readability < 75.0 {
            recommendations
                .push("Improve code readability with better naming and formatting".to_string());
        }

        if metrics.maintainability < 75.0 {
            recommendations
                .push("Break down complex functions into smaller, modular pieces".to_string());
        }

        if metrics.security < 90.0 {
            recommendations
                .push("Review security practices, especially error handling".to_string());
        }

        if metrics.test_coverage < 50.0 {
            recommendations.push("Add comprehensive unit tests".to_string());
        }

        if metrics.documentation < 60.0 {
            recommendations.push("Add documentation comments for public APIs".to_string());
        }

        recommendations
    }

    /// Validate code against standards
    pub fn validate(&self, code: &str) -> Result<()> {
        let metrics = self.analyse(code)?;

        if self.strict_mode && metrics.overall_score < self.min_overall_score {
            return Err(AgenticError::ExcellenceFailed(format!(
                "Code quality score {:.1} is below minimum {:.1}. Issues: {:?}",
                metrics.overall_score, self.min_overall_score, metrics.recommendations
            )));
        }

        Ok(())
    }

    /// Set strict mode
    pub fn set_strict_mode(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Set minimum score
    pub fn set_min_score(&mut self, score: f64) {
        self.min_overall_score = score;
    }
}

impl Default for ExcellenceEnforcer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_metrics() {
        let mut metrics = QualityMetrics::new();
        metrics.readability = 90.0;
        metrics.maintainability = 85.0;
        metrics.performance = 80.0;
        metrics.security = 95.0;
        metrics.test_coverage = 70.0;
        metrics.documentation = 75.0;
        metrics.calculate_overall();

        assert!(metrics.overall_score > 0.0);
        assert!(metrics.is_good());
    }

    #[test]
    fn test_excellence_enforcer() {
        let enforcer = ExcellenceEnforcer::new();
        let code = r#"
            /// A test function
            fn test_function() {
                println!("Hello, world!");
            }
            
            #[cfg(test)]
            mod tests {
                #[test]
                fn test_it() {
                    assert!(true);
                }
            }
        "#;

        let metrics = enforcer.analyse(code);
        assert!(metrics.is_ok());
    }
}
