//! Extension bridge for connecting VCode extensions to MCP

use anyhow::Result;
use fusion_vscode_runtime::{ActivationContext, Extension, ExtensionHost};
use std::collections::HashMap;
use std::path::PathBuf;

/// Bridge between VS Code extensions and MCP protocol
pub struct ExtensionMcpBridge {
    extension_host: Option<ExtensionHost>,
    extension_dir: PathBuf,
    active_extensions: HashMap<String, ExtensionInfo>,
}

#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub tools_provided: Vec<String>,
}

impl ExtensionMcpBridge {
    pub fn new() -> Self {
        Self {
            extension_host: None,
            extension_dir: PathBuf::from("~/.fusion/extensions"),
            active_extensions: HashMap::new(),
        }
    }

    /// Initialize the extension bridge
    pub async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing extension bridge");

        // Expand extension directory path
        let extension_dir = shellexpand::tilde(&self.extension_dir.to_string_lossy()).into_owned();
        let extension_dir = PathBuf::from(extension_dir);

        // Create extension host
        let context = ActivationContext {
            workspace_root: std::env::current_dir()?,
            extension_path: extension_dir.clone(),
            storage_path: extension_dir.join("storage"),
        };

        let mut host = ExtensionHost::new(context);

        // Load extensions from directory
        if extension_dir.exists() {
            self.load_extensions_from_dir(&mut host, &extension_dir)
                .await?;
        }

        // Activate all extensions
        host.activate_all().await?;

        self.extension_host = Some(host);
        tracing::info!(
            "Extension bridge initialized with {} extensions",
            self.active_extensions.len()
        );

        Ok(())
    }

    /// Load extensions from a directory
    async fn load_extensions_from_dir(
        &mut self,
        host: &mut ExtensionHost,
        dir: &PathBuf,
    ) -> Result<()> {
        if !dir.exists() {
            tracing::warn!("Extension directory does not exist: {:?}", dir);
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                match host.load_extension(entry.path()).await {
                    Ok(_) => {
                        tracing::info!("Loaded extension from {:?}", entry.path());
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load extension: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Register an extension's tools with MCP
    pub async fn register_extension_tools(&mut self, extension: &Extension) -> Result<()> {
        let tools = self.extract_tools_from_extension(extension);

        let info = ExtensionInfo {
            id: extension.id.clone(),
            name: extension.name.clone(),
            version: extension.version.clone(),
            tools_provided: tools,
        };

        self.active_extensions.insert(extension.id.clone(), info);
        Ok(())
    }

    /// Extract MCP tools from an extension
    fn extract_tools_from_extension(&self, extension: &Extension) -> Vec<String> {
        // Map extension capabilities to MCP tools
        extension
            .capabilities
            .iter()
            .map(|cap| format!("{}.{:?}", extension.id, cap))
            .collect()
    }

    /// Execute a tool provided by an extension
    pub async fn execute_extension_tool(
        &self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value> {
        tracing::debug!("Executing extension tool: {}", tool_name);

        // Parse tool name to get extension ID and method
        let parts: Vec<&str> = tool_name.split('.').collect();
        if parts.len() < 2 {
            return Err(anyhow::anyhow!("Invalid tool name format"));
        }

        let extension_id = parts[0];
        let method = parts[1..].join(".");

        // Find the extension
        if let Some(_ext_info) = self.active_extensions.get(extension_id) {
            // Execute the tool
            // In a real implementation, this would call into the extension host
            tracing::info!(
                "Would execute {}.{} with args: {}",
                extension_id,
                method,
                args
            );
            Ok(serde_json::json!({ "success": true, "result": "Tool executed" }))
        } else {
            Err(anyhow::anyhow!("Extension not found: {}", extension_id))
        }
    }

    /// Get list of all available tools from extensions
    pub fn get_available_tools(&self) -> Vec<String> {
        self.active_extensions
            .values()
            .flat_map(|ext| ext.tools_provided.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_bridge_creation() {
        let bridge = ExtensionMcpBridge::new();
        assert_eq!(bridge.active_extensions.len(), 0);
    }
}
