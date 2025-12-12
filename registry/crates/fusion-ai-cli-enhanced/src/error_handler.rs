//! Enhanced error handling with AI-generated explanations

use crate::{context::CliContext, CliError, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

/// An AI-generated error explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorExplanation {
    /// The original error message
    pub error: String,

    /// Human-friendly explanation
    pub explanation: String,

    /// Suggested fixes
    pub suggestions: Vec<String>,

    /// Related documentation links
    pub docs: Vec<String>,

    /// Example of correct usage
    pub example: Option<String>,
}

/// Enhanced error handler
pub struct EnhancedErrorHandler {
    known_errors: Vec<ErrorPattern>,
}

#[derive(Debug, Clone)]
struct ErrorPattern {
    pattern: String,
    explanation: String,
    suggestions: Vec<String>,
    docs: Vec<String>,
}

impl EnhancedErrorHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            known_errors: Vec::new(),
        };
        handler.load_error_patterns();
        handler
    }

    fn load_error_patterns(&mut self) {
        self.known_errors.push(ErrorPattern {
            pattern: "command not found".to_string(),
            explanation: "The command you entered is not recognized. This usually means the command doesn't exist or there's a typo.".to_string(),
            suggestions: vec![
                "Check the spelling of your command".to_string(),
                "Run 'help' to see available commands".to_string(),
                "Try using natural language, e.g., 'build the project'".to_string(),
            ],
            docs: vec!["https://docs.fusion-lang.org/cli/commands".to_string()],
        });

        self.known_errors.push(ErrorPattern {
            pattern: "permission denied".to_string(),
            explanation: "You don't have sufficient permissions to perform this operation."
                .to_string(),
            suggestions: vec![
                "Try running with administrator/sudo privileges".to_string(),
                "Check file permissions".to_string(),
                "Ensure you own the target directory".to_string(),
            ],
            docs: vec!["https://docs.fusion-lang.org/troubleshooting/permissions".to_string()],
        });

        self.known_errors.push(ErrorPattern {
            pattern: "compilation failed".to_string(),
            explanation: "The project failed to compile due to errors in the source code."
                .to_string(),
            suggestions: vec![
                "Review the error messages above".to_string(),
                "Check for syntax errors".to_string(),
                "Ensure all dependencies are installed".to_string(),
                "Try 'clean' then 'build' again".to_string(),
            ],
            docs: vec!["https://docs.fusion-lang.org/compiler/errors".to_string()],
        });

        self.known_errors.push(ErrorPattern {
            pattern: "dependency".to_string(),
            explanation: "There's an issue with one or more project dependencies.".to_string(),
            suggestions: vec![
                "Run 'update' to refresh dependencies".to_string(),
                "Check your Cargo.toml for version conflicts".to_string(),
                "Clear the cache and reinstall".to_string(),
            ],
            docs: vec!["https://docs.fusion-lang.org/dependencies".to_string()],
        });
    }

    /// Generate an explanation for an error
    pub fn explain(&self, error: &str, _ctx: &CliContext) -> Result<ErrorExplanation> {
        let error_lower = error.to_lowercase();

        // Find matching pattern
        for pattern in &self.known_errors {
            if error_lower.contains(&pattern.pattern) {
                return Ok(ErrorExplanation {
                    error: error.to_string(),
                    explanation: pattern.explanation.clone(),
                    suggestions: pattern.suggestions.clone(),
                    docs: pattern.docs.clone(),
                    example: self.generate_example(&pattern.pattern),
                });
            }
        }

        // Generic explanation if no pattern matches
        Ok(ErrorExplanation {
            error: error.to_string(),
            explanation: "An unexpected error occurred.".to_string(),
            suggestions: vec![
                "Check the error message for details".to_string(),
                "Try running with --verbose for more information".to_string(),
                "Consult the documentation".to_string(),
            ],
            docs: vec!["https://docs.fusion-lang.org/troubleshooting".to_string()],
            example: None,
        })
    }

    fn generate_example(&self, error_type: &str) -> Option<String> {
        match error_type {
            "command not found" => Some("fusion build --release".to_string()),
            "compilation failed" => Some("fusion clean && fusion build".to_string()),
            "dependency" => Some("fusion update".to_string()),
            _ => None,
        }
    }

    /// Format error explanation for display
    pub fn format_explanation(&self, explanation: &ErrorExplanation) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "Error Explanation:".bold().red()));
        output.push_str(&format!("{}\n\n", explanation.explanation));

        if !explanation.suggestions.is_empty() {
            output.push_str(&format!("{}\n", "Suggestions:".bold().yellow()));
            for (i, suggestion) in explanation.suggestions.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, suggestion));
            }
            output.push('\n');
        }

        if let Some(ref example) = explanation.example {
            output.push_str(&format!("{}\n", "Example:".bold().green()));
            output.push_str(&format!("  {}\n\n", example.cyan()));
        }

        if !explanation.docs.is_empty() {
            output.push_str(&format!("{}\n", "Documentation:".bold().blue()));
            for doc in &explanation.docs {
                output.push_str(&format!("  {}\n", doc.underline()));
            }
        }

        output
    }
}

impl Default for EnhancedErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_explanation() {
        let handler = EnhancedErrorHandler::new();
        let ctx = CliContext::new();
        let result = handler.explain("command not found: buidl", &ctx);
        assert!(result.is_ok());
        let explanation = result.unwrap();
        assert!(!explanation.suggestions.is_empty());
    }

    #[test]
    fn test_format_explanation() {
        let handler = EnhancedErrorHandler::new();
        let explanation = ErrorExplanation {
            error: "test error".to_string(),
            explanation: "This is a test".to_string(),
            suggestions: vec!["Try this".to_string()],
            docs: vec![],
            example: Some("example command".to_string()),
        };

        let formatted = handler.format_explanation(&explanation);
        assert!(formatted.contains("test"));
    }
}
