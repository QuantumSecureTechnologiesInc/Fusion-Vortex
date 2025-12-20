//! Fusion Standard Library Extensions
//!
//! Provides common error types and utilities for the Fusion ecosystem.

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum StdError {
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        #[error("Parse error: {0}")]
        Parse(String),
        #[error("Configuration error: {0}")]
        Config(String),
        #[error("Runtime error: {0}")]
        Runtime(String),
        #[error("Core error: {0}")]
        Core(#[from] fusion_core::FusionError),
        #[error("Serialization error: {0}")]
        Serialization(String),
        #[error("Permission denied: {0}")]
        PermissionDenied(String),
    }

    pub type StdResult<T> = Result<T, StdError>;
}

pub use error::{StdError, StdResult};

pub mod io;
