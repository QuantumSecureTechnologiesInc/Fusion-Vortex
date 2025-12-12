//! Fusion Terminal Browser
//!
//! A terminal-based browser using the Blink rendering engine and WebGPU
//! for GPU-accelerated rendering to the terminal.
//!
//! # Features
//! - Blink/Chromium rendering engine
//! - WebGPU support for GPU-accelerated graphics
//! - Terminal-based UI with full mouse and keyboard support
//! - Image-to-ASCII/Unicode conversion
//! - Session management
//! - CLI integration
//!
//! # Example
//! ```no_run
//! use fusion_terminal_browser::{Browser, BrowserConfig};
//!
//! let config = BrowserConfig::default();
//! let browser = Browser::new(config)?;
//! browser.navigate("https://example.com")?;
//! browser.render()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod browser;
pub mod config;
pub mod engine;
pub mod error;
pub mod renderer;
pub mod session;
pub mod terminal;
pub mod webgpu;

// Re-exports
pub use browser::Browser;
pub use config::BrowserConfig;
pub use error::{BrowserError, Result};
pub use session::Session;

/// Browser version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Maximum supported terminal dimensions
pub const MAX_TERMINAL_WIDTH: u16 = 1024;
pub const MAX_TERMINAL_HEIGHT: u16 = 768;

/// Default user agent string
pub const USER_AGENT: &str = concat!(
    "Fusion-Terminal-Browser/",
    env!("CARGO_PKG_VERSION"),
    " (Blink; WebGPU)"
);
