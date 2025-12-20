//! Enhanced MCP (Model Context Protocol) Server Implementation
//!
//! This module provides a full-featured MCP server that can host VS Code extensions,
//! provide context to AI models, and serve as a bridge between the CLI and AI services.

pub mod client;
pub mod context_provider;
pub mod extension_bridge;
pub mod protocol;
pub mod servers;
pub mod tool_registry;

pub use client::McpClient;
pub use context_provider::ContextProvider;
pub use extension_bridge::ExtensionMcpBridge;
pub use protocol::*;
pub use servers::{FilesystemServer, GitHubServer, WebServer};
pub use tool_registry::ToolRegistry;

/// MCP library version
pub const MCP_LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

/// MCP Protocol version
pub const MCP_PROTOCOL_VERSION: &str = "2024-11-05";

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main MCP server that coordinates all capabilities
pub struct McpServer {
    context_provider: Arc<ContextProvider>,
    tool_registry: Arc<ToolRegistry>,
    extension_bridge: Arc<RwLock<ExtensionMcpBridge>>,
    config: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
    pub capabilities: ServerCapabilities,
}

#[derive(Debug, Clone, Default)]
pub struct ServerCapabilities {
    pub tools: bool,
    pub resources: bool,
    pub prompts: bool,
    pub extensions: bool,
}

impl McpServer {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            context_provider: Arc::new(ContextProvider::new()),
            tool_registry: Arc::new(ToolRegistry::new()),
            extension_bridge: Arc::new(RwLock::new(ExtensionMcpBridge::new())),
            config,
        }
    }

    /// Start the MCP server
    pub async fn start(&self, addr: &str) -> Result<()> {
        tracing::info!(
            "Starting MCP server {} v{}",
            self.config.name,
            self.config.version
        );
        tracing::info!("Server address: {}", addr);
        tracing::info!("Capabilities: {:?}", self.config.capabilities);

        // Initialize extension bridge if enabled
        if self.config.capabilities.extensions {
            let mut bridge = self.extension_bridge.write().await;
            bridge.initialize().await?;
        }

        Ok(())
    }

    /// Get context provider
    pub fn context_provider(&self) -> Arc<ContextProvider> {
        Arc::clone(&self.context_provider)
    }

    /// Get tool registry
    pub fn tool_registry(&self) -> Arc<ToolRegistry> {
        Arc::clone(&self.tool_registry)
    }

    /// Get extension bridge
    pub fn extension_bridge(&self) -> Arc<RwLock<ExtensionMcpBridge>> {
        Arc::clone(&self.extension_bridge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_server_creation() {
        let config = ServerConfig {
            name: "test-server".to_string(),
            version: "1.0.0".to_string(),
            capabilities: ServerCapabilities {
                tools: true,
                resources: true,
                prompts: true,
                extensions: true,
            },
        };

        let server = McpServer::new(config);
        assert_eq!(server.config.name, "test-server");
    }
}
