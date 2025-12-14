//! # Fusion Net
//!
//! High-performance networking primitives using fusion_runtime_core.
//!
//! Replaces Tokio's networking with direct HAL access for maximum performance.

pub mod tcp;

pub use tcp::{TcpListener, TcpStream};
