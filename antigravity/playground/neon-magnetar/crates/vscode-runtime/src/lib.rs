//! VS Code Extension Runtime for Fusion CLI
//!
//! This module enables running VS Code extensions within the Fusion CLI environment,
//! providing a bridge between the CLI and the VS Code extension API.

pub mod extension_host;
pub mod extension_loader;
pub mod node_bridge;
pub mod vscode_api;
pub mod workspace;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a VS Code extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub entry_point: PathBuf,
    pub capabilities: Vec<ExtensionCapability>,
}

/// Extension capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionCapability {
    LanguageServer,
    Formatter,
    Linter,
    Debugger,
    CodeAction,
    Completion,
    Hover,
    Definition,
    References,
}

/// Extension activation context
#[derive(Debug, Clone)]
pub struct ActivationContext {
    pub workspace_root: PathBuf,
    pub extension_path: PathBuf,
    pub storage_path: PathBuf,
}

/// Main extension host that manages all loaded extensions
pub struct ExtensionHost {
    extensions: Vec<Extension>,
    context: ActivationContext,
}

impl ExtensionHost {
    pub fn new(context: ActivationContext) -> Self {
        Self {
            extensions: Vec::new(),
            context,
        }
    }

    /// Load an extension from a directory
    pub async fn load_extension(&mut self, path: PathBuf) -> Result<()> {
        let extension = extension_loader::load_extension(&path).await?;
        self.extensions.push(extension);
        Ok(())
    }

    /// Activate all loaded extensions
    pub async fn activate_all(&mut self) -> Result<()> {
        for extension in &self.extensions {
            tracing::info!("Activating extension: {}", extension.name);
            self.activate_extension(&extension.id).await?;
        }
        Ok(())
    }

    /// Activate a specific extension by ID
    pub async fn activate_extension(&self, id: &str) -> Result<()> {
        let extension = self
            .extensions
            .iter()
            .find(|e| e.id == id)
            .ok_or_else(|| anyhow::anyhow!("Extension not found: {}", id))?;

        extension_host::activate(extension, &self.context).await
    }

    /// Get all active extensions
    pub fn get_extensions(&self) -> &[Extension] {
        &self.extensions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_host_creation() {
        let context = ActivationContext {
            workspace_root: PathBuf::from("/workspace"),
            extension_path: PathBuf::from("/extensions"),
            storage_path: PathBuf::from("/storage"),
        };
        let host = ExtensionHost::new(context);
        assert_eq!(host.extensions.len(), 0);
    }
}
