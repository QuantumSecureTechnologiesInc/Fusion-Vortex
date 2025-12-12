//! Error types for the Fusion Monolith Core
//!
//! This module provides comprehensive error handling for all monolith operations.

use std::fmt;

/// The main error type for Fusion Monolith operations
#[derive(Debug, Clone)]
pub enum MonolithError {
    /// Error during dependency resolution
    Resolution(String),
    /// Security vulnerability detected
    Security(String),
    /// Compilation error
    Compilation(String),
    /// Test failure
    TestFailure {
        passed: usize,
        failed: usize,
        message: String,
    },
    /// Runtime execution error
    Runtime(String),
    /// LSP server error
    Lsp(String),
    /// Configuration error
    Config(String),
    /// I/O error
    Io(String),
    /// Agent error
    Agent { role: String, message: String },
    /// State lock poisoned
    StatePoisoned,
    /// Invalid command
    InvalidCommand(String),
    /// Missing artifact
    MissingArtifact(String),
    /// CUDA/GPU error (when cuda feature is enabled)
    Gpu(String),
}

impl fmt::Display for MonolithError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolution(msg) => write!(f, "Resolution error: {}", msg),
            Self::Security(msg) => write!(f, "Security vulnerability: {}", msg),
            Self::Compilation(msg) => write!(f, "Compilation error: {}", msg),
            Self::TestFailure {
                passed,
                failed,
                message,
            } => {
                write!(
                    f,
                    "Test failure: {} passed, {} failed - {}",
                    passed, failed, message
                )
            }
            Self::Runtime(msg) => write!(f, "Runtime error: {}", msg),
            Self::Lsp(msg) => write!(f, "LSP error: {}", msg),
            Self::Config(msg) => write!(f, "Configuration error: {}", msg),
            Self::Io(msg) => write!(f, "I/O error: {}", msg),
            Self::Agent { role, message } => write!(f, "Agent {} error: {}", role, message),
            Self::StatePoisoned => write!(f, "Internal state lock was poisoned"),
            Self::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            Self::MissingArtifact(name) => write!(f, "Missing artifact: {}", name),
            Self::Gpu(msg) => write!(f, "GPU error: {}", msg),
        }
    }
}

impl std::error::Error for MonolithError {}

impl From<std::io::Error> for MonolithError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<serde_json::Error> for MonolithError {
    fn from(err: serde_json::Error) -> Self {
        Self::Config(format!("JSON error: {}", err))
    }
}

/// Result type alias for Monolith operations
pub type MonolithResult<T> = Result<T, MonolithError>;

/// Severity levels for diagnostics and errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational hint
    Hint,
    /// Warning (non-blocking)
    Warning,
    /// Error (blocking)
    Error,
    /// Critical security issue
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hint => write!(f, "hint"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MonolithError::Security("CVE-2024-0001".to_string());
        assert!(err.to_string().contains("Security vulnerability"));
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::Error);
        assert!(Severity::Error > Severity::Warning);
        assert!(Severity::Warning > Severity::Hint);
    }
}
