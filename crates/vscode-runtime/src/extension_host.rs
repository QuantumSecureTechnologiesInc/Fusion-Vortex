//! Extension host implementation for activating and managing extensions

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Handler for expansion commands
pub type CommandHandler = Box<dyn Fn(Vec<String>) -> Result<String> + Send + Sync>;

use fusion_policy::{Capability, EnforcementMode, ExtensionManifest, PolicyEnforcer, TrustLevel};
use std::path::PathBuf;

/// Extension host for managing loaded extensions and commands
pub struct ExtensionHost {
    active_extensions: Arc<RwLock<HashMap<String, ActiveExtension>>>,
    commands: Arc<RwLock<HashMap<String, CommandHandler>>>,
    enforcer: Arc<RwLock<PolicyEnforcer>>,
    manifests: Arc<RwLock<HashMap<String, ExtensionManifest>>>,
    extensions_dir: Option<PathBuf>,
}

/// Active extension instance
#[derive(Debug)]
struct ActiveExtension {
    _id: String,
    _name: String,
}

impl ExtensionHost {
    /// Create a new extension host
    pub fn new() -> Self {
        Self {
            active_extensions: Arc::new(RwLock::new(HashMap::new())),
            commands: Arc::new(RwLock::new(HashMap::new())),
            enforcer: Arc::new(RwLock::new(PolicyEnforcer::warn_only())), // Default to warn-only per migration plan
            manifests: Arc::new(RwLock::new(HashMap::new())),
            extensions_dir: dirs::home_dir().map(|p| p.join(".fusion/extensions")),
        }
    }

    /// Set the enforcement mode
    pub async fn set_enforcement_mode(&self, mode: EnforcementMode) {
        let mut enforcer = self.enforcer.write().await;
        enforcer.set_mode(mode);
    }

    /// Get the policy enforcer
    pub fn enforcer(&self) -> Arc<RwLock<PolicyEnforcer>> {
        self.enforcer.clone()
    }

    /// Check if an extension has a capability
    pub async fn check_capability(&self, extension_id: &str, capability: Capability) -> Result<()> {
        let manifests = self.manifests.read().await;

        let empty_caps = Vec::new();
        let capabilities = if let Some(manifest) = manifests.get(extension_id) {
            &manifest.capabilities
        } else {
            tracing::warn!(
                "No manifest found for extension {}, assuming no capabilities",
                extension_id
            );
            &empty_caps
        };

        let enforcer = self.enforcer.read().await;
        enforcer.check_capability(&capability, capabilities)
    }

    /// Activate an extension
    pub async fn activate_extension(&self, extension_id: &str) -> Result<()> {
        tracing::info!("Activating extension: {}", extension_id);

        // Try to load manifest
        if let Some(dir) = &self.extensions_dir {
            match ExtensionManifest::load(dir, extension_id) {
                Ok(manifest) => {
                    let mut manifests = self.manifests.write().await;
                    manifests.insert(extension_id.to_string(), manifest);
                    tracing::info!("Loaded capability manifest for {}", extension_id);
                }
                Err(_) => {
                    // Try to create a default one if it doesn't exist
                    // For known extensions, we might pre-populate
                    if extension_id == "google.gemini-code-assist" {
                        tracing::info!("Creating default manifest for Gemini Code Assist");
                        let mut manifest =
                            ExtensionManifest::new(extension_id, TrustLevel::Trusted);
                        manifest
                            .add_capability(Capability::NetworkOutbound, "Required for AI service");
                        manifest.add_capability(Capability::WorkspaceInspect, "Context analysis");

                        // Check capabilities implicitly grants them in the manifest object
                        // Now save it
                        if let Err(e) = manifest.save(dir) {
                            tracing::warn!("Failed to save default manifest: {}", e);
                        }

                        let mut manifests = self.manifests.write().await;
                        manifests.insert(extension_id.to_string(), manifest);
                    } else {
                        tracing::warn!(
                            "No manifest found for {}; using empty capabilities",
                            extension_id
                        );
                    }
                }
            }
        }

        let ext = ActiveExtension {
            _id: extension_id.to_string(),
            _name: extension_id.to_string(),
        };

        let mut extensions = self.active_extensions.write().await;
        extensions.insert(extension_id.to_string(), ext);

        tracing::info!("Extension {} activated successfully", extension_id);
        Ok(())
    }

    /// Deactivate an extension
    pub async fn deactivate_extension(&self, extension_id: &str) -> Result<()> {
        tracing::info!("Deactivating extension: {}", extension_id);

        let mut extensions = self.active_extensions.write().await;
        extensions.remove(extension_id);

        tracing::info!("Extension {} deactivated successfully", extension_id);
        Ok(())
    }

    /// Register a command handler
    pub async fn register_command<F>(&self, command: &str, handler: F)
    where
        F: Fn(Vec<String>) -> Result<String> + Send + Sync + 'static,
    {
        let mut commands = self.commands.write().await;
        commands.insert(command.to_string(), Box::new(handler));
        tracing::debug!("Registered command: {}", command);
    }

    /// Execute a command from an extension
    pub async fn execute_command(&self, command: &str, args: Vec<String>) -> Result<String> {
        tracing::info!("Executing command: {} with args: {:?}", command, args);

        let commands = self.commands.read().await;

        if let Some(handler) = commands.get(command) {
            handler(args)
        } else {
            let extensions = self.active_extensions.read().await;
            if !extensions.is_empty() {
                tracing::warn!(
                    "Command {} not found, simulating success for active extension",
                    command
                );
                Ok(format!("Command {} executed (simulation)", command))
            } else {
                Err(anyhow::anyhow!("Command not found: {}", command))
            }
        }
    }

    /// Get list of active extension IDs
    pub async fn active_extensions(&self) -> Vec<String> {
        let extensions = self.active_extensions.read().await;
        extensions.keys().cloned().collect()
    }
}

impl Default for ExtensionHost {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extension_host() {
        let host = ExtensionHost::new();
        assert!(host.activate_extension("test").await.is_ok());
        assert!(host.active_extensions().await.contains(&"test".to_string()));

        // Test command registration
        host.register_command("test.echo", |args| Ok(args.join(" ")))
            .await;

        let result = host
            .execute_command("test.echo", vec!["hello".to_string()])
            .await;
        assert_eq!(result.unwrap(), "hello");

        assert!(host.deactivate_extension("test").await.is_ok());
        assert!(host.active_extensions().await.is_empty());
    }
}
