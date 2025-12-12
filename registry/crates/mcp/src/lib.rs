pub mod client;
pub mod protocol;
pub mod servers;

pub use client::McpClient;
pub use protocol::*;
pub use servers::{FilesystemServer, GitHubServer, WebServer};

pub mod extension_bridge;
pub use extension_bridge::ExtensionMcpBridge;

/// MCP library version
pub const MCP_LIB_VERSION: &str = env!("CARGO_PKG_VERSION");
