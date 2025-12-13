//! Extension capability manifest definitions

use crate::capability::Capability;
use crate::trust::TrustLevel;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Extension capability manifest
/// Stored at: ~/.fusion/extensions/<publisher>.<name>/capabilities.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    /// Extension identifier (publisher.name)
    pub extension: String,
    /// Trust level for this extension
    pub trust: TrustLevel,
    /// Capabilities granted to this extension
    pub capabilities: Vec<Capability>,
    /// Optional capability justifications
    #[serde(default)]
    pub justifications: Vec<CapabilityJustification>,
    /// Manifest version
    #[serde(default)]
    pub version: u32,
}

/// Justification for why a capability is needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityJustification {
    pub capability: Capability,
    pub reason: String,
}

impl ExtensionManifest {
    /// Create a new manifest with minimal capabilities
    pub fn new(extension: impl Into<String>, trust: TrustLevel) -> Self {
        Self {
            extension: extension.into(),
            trust,
            capabilities: vec![],
            justifications: vec![],
            version: 1,
        }
    }

    /// Create a manifest with specific capabilities
    pub fn with_capabilities(
        extension: impl Into<String>,
        trust: TrustLevel,
        capabilities: Vec<Capability>,
    ) -> Self {
        Self {
            extension: extension.into(),
            trust,
            capabilities,
            justifications: vec![],
            version: 1,
        }
    }

    /// Add a capability with justification
    pub fn add_capability(&mut self, capability: Capability, reason: impl Into<String>) {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability.clone());
        }
        self.justifications.push(CapabilityJustification {
            capability,
            reason: reason.into(),
        });
    }

    /// Check if a capability is granted
    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Save manifest to disk
    pub fn save(&self, base_dir: &Path) -> Result<()> {
        let manifest_path = Self::manifest_path(base_dir, &self.extension);

        // Ensure parent directory exists
        if let Some(parent) = manifest_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create manifest directory: {:?}", parent))?;
        }

        let json = serde_json::to_string_pretty(self).context("Failed to serialize manifest")?;

        std::fs::write(&manifest_path, json)
            .with_context(|| format!("Failed to write manifest to {:?}", manifest_path))?;

        Ok(())
    }

    /// Load manifest from disk
    pub fn load(base_dir: &Path, extension: &str) -> Result<Self> {
        let manifest_path = Self::manifest_path(base_dir, extension);

        let json = std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("Failed to read manifest from {:?}", manifest_path))?;

        let manifest: Self =
            serde_json::from_str(&json).context("Failed to deserialize manifest")?;

        Ok(manifest)
    }

    /// Get the path to the manifest file
    fn manifest_path(base_dir: &Path, extension: &str) -> PathBuf {
        base_dir.join(extension).join("capabilities.json")
    }

    /// Check if manifest exists on disk
    pub fn exists(base_dir: &Path, extension: &str) -> bool {
        Self::manifest_path(base_dir, extension).exists()
    }
}

/// Capability manifest for the entire system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityManifest {
    /// Global policy settings
    pub policy: GlobalPolicy,
    /// Per-extension manifests
    pub extensions: Vec<ExtensionManifest>,
}

/// Global policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPolicy {
    /// Enforcement mode
    pub enforcement_mode: String,
    /// Default trust level for new extensions
    pub default_trust: TrustLevel,
    /// Automatically grant safe capabilities
    pub auto_grant_safe: bool,
}

impl Default for GlobalPolicy {
    fn default() -> Self {
        Self {
            enforcement_mode: "strict".to_string(),
            default_trust: TrustLevel::Unverified,
            auto_grant_safe: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manifest() {
        let manifest = ExtensionManifest::new("publisher.extension", TrustLevel::Verified);
        assert_eq!(manifest.extension, "publisher.extension");
        assert_eq!(manifest.trust, TrustLevel::Verified);
        assert!(manifest.capabilities.is_empty());
    }

    #[test]
    fn test_add_capability() {
        let mut manifest = ExtensionManifest::new("test.extension", TrustLevel::Verified);

        manifest.add_capability(Capability::FilesystemRead, "Needs to read config files");

        assert!(manifest.has_capability(&Capability::FilesystemRead));
        assert_eq!(manifest.justifications.len(), 1);
    }

    #[test]
    fn test_serialization() {
        let manifest = ExtensionManifest::with_capabilities(
            "test.ext",
            TrustLevel::Verified,
            vec![Capability::FilesystemRead, Capability::NetworkOutbound],
        );

        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: ExtensionManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.extension, "test.ext");
        assert_eq!(deserialized.capabilities.len(), 2);
    }
}
