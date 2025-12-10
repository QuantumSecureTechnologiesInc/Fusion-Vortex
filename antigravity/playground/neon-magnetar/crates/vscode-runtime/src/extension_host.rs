//! Extension host implementation for activating and managing extensions

use crate::{ActivationContext, Extension};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Extension host runtime state
pub struct ExtensionHostRuntime {
    active_extensions: Arc<RwLock<HashMap<String, ExtensionInstance>>>,
}

/// Running instance of an extension
pub struct ExtensionInstance {
    pub id: String,
    pub context: ActivationContext,
    pub api: Arc<dyn ExtensionAPI + Send + Sync>,
}

/// Extension API trait
#[async_trait::async_trait]
pub trait ExtensionAPI {
    async fn on_activate(&self) -> Result<()>;
    async fn on_deactivate(&self) -> Result<()>;
    async fn execute_command(
        &self,
        command: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value>;
}

impl ExtensionHostRuntime {
    pub fn new() -> Self {
        Self {
            active_extensions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_extension(&self, id: String, instance: ExtensionInstance) -> Result<()> {
        let mut extensions = self.active_extensions.write().await;
        extensions.insert(id, instance);
        Ok(())
    }

    pub async fn get_extension(&self, id: &str) -> Option<Arc<dyn ExtensionAPI + Send + Sync>> {
        let extensions = self.active_extensions.read().await;
        extensions.get(id).map(|e| Arc::clone(&e.api))
    }
}

/// Activate an extension
pub async fn activate(extension: &Extension, context: &ActivationContext) -> Result<()> {
    tracing::info!(
        "Activating extension: {} v{}",
        extension.name,
        extension.version
    );

    // Load extension manifest
    let manifest_path = extension
        .entry_point
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid extension path"))?
        .join("package.json");

    if !manifest_path.exists() {
        return Err(anyhow::anyhow!("Extension manifest not found"));
    }

    // Parse manifest
    let manifest_content = tokio::fs::read_to_string(&manifest_path).await?;
    let _manifest: serde_json::Value = serde_json::from_str(&manifest_content)?;

    // Initialize extension runtime environment
    initialize_extension_env(extension, context).await?;

    tracing::info!("Extension {} activated successfully", extension.name);
    Ok(())
}

/// Initialize extension runtime environment
async fn initialize_extension_env(
    extension: &Extension,
    context: &ActivationContext,
) -> Result<()> {
    // Create extension storage directory
    let storage_dir = context.storage_path.join(&extension.id);
    tokio::fs::create_dir_all(&storage_dir).await?;

    // Set up extension workspace
    tracing::debug!("Extension workspace: {:?}", context.workspace_root);
    tracing::debug!("Extension storage: {:?}", storage_dir);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_extension_host_runtime() {
        let runtime = ExtensionHostRuntime::new();
        assert!(runtime.get_extension("test").await.is_none());
    }
}
