// src/network/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Cryptographic error: {0}")]
    Crypto(String),

    #[error("Handshake failed: {0}")]
    Handshake(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Rate limit exceeded")]
    RateLimit,
}
