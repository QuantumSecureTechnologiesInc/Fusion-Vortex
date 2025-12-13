//! Error types for the terminal browser

use thiserror::Error;

/// Result type alias for browser operations
pub type Result<T> = std::result::Result<T, BrowserError>;

/// Browser error types
#[derive(Debug, Error)]
pub enum BrowserError {
    /// Chrome/Chromium engine error
    #[error("Browser engine error: {0}")]
    Engine(String),

    /// WebGPU-related error
    #[error("WebGPU error: {0}")]
    WebGpu(String),

    /// Terminal rendering error
    #[error("Terminal error: {0}")]
    Terminal(#[from] std::io::Error),

    /// Navigation error
    #[error("Navigation error: {0}")]
    Navigation(String),

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    /// Session management error
    #[error("Session error: {0}")]
    Session(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Rendering error
    #[error("Rendering error: {0}")]
    Rendering(String),

    /// Image processing error
    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    /// JavaScript execution error
    #[error("JavaScript error: {0}")]
    JavaScript(String),

    /// Timeout error
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// Generic error
    #[error("Browser error: {0}")]
    Generic(String),
}

// Chromiumoxide error conversion
impl From<chromiumoxide::error::CdpError> for BrowserError {
    fn from(err: chromiumoxide::error::CdpError) -> Self {
        BrowserError::Engine(err.to_string())
    }
}

impl From<anyhow::Error> for BrowserError {
    fn from(err: anyhow::Error) -> Self {
        BrowserError::Generic(err.to_string())
    }
}
