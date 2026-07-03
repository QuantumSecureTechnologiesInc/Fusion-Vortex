//! Extension loader for discovering and loading VS Code extensions

use crate::Extension;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// VS Code extension manifest (package.json)
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub publisher: Option<String>,
    pub main: Option<String>,
    pub activationEvents: Option<Vec<String>>,
    pub contributes: Option<serde_json::Value>,
    pub engines: EngineRequirements,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EngineRequirements {
    #[serde(rename = "vscode")]
    pub vscode: String,
}

/// Load an extension from a directory
pub async fn load_extension(path: &Path) -> Result<Extension> {
    let manifest_path = path.join("package.json");

    if !manifest_path.exists() {
        return Err(anyhow::anyhow!(
            "No package.json found in extension directory"
        ));
    }

    let content = tokio::fs::read_to_string(&manifest_path).await?;
    let manifest: ExtensionManifest = serde_json::from_str(&content)?;

    // Determine entry point
    let entry_point = if let Some(main) = &manifest.main {
        path.join(main)
    } else {
        path.join("out/extension.js")
    };

    // Parse capabilities from contributes
    let capabilities = parse_capabilities(&manifest);

    let extension_id = format!(
        "{}.{}",
        manifest.publisher.as_deref().unwrap_or("unknown"),
        manifest.name
    );

    Ok(Extension {
        id: extension_id,
        name: manifest.display_name.unwrap_or(manifest.name),
        version: manifest.version,
        entry_point,
        capabilities,
    })
}

/// Parse extension capabilities from manifest
fn parse_capabilities(manifest: &ExtensionManifest) -> Vec<crate::ExtensionCapability> {
    let mut capabilities = Vec::new();

    if let Some(contributes) = &manifest.contributes {
        if contributes.get("languages").is_some() {
            capabilities.push(crate::ExtensionCapability::LanguageServer);
        }
        if contributes.get("grammars").is_some() {
            capabilities.push(crate::ExtensionCapability::LanguageServer);
        }
        if contributes.get("debuggers").is_some() {
            capabilities.push(crate::ExtensionCapability::Debugger);
        }
        if contributes.get("commands").is_some() {
            capabilities.push(crate::ExtensionCapability::CodeAction);
        }
    }

    capabilities
}

/// Discover all extensions in a directory
pub async fn discover_extensions(directory: &Path) -> Result<Vec<Extension>> {
    let mut extensions = Vec::new();

    if !directory.exists() {
        tracing::warn!("Extension directory does not exist: {:?}", directory);
        return Ok(extensions);
    }

    let mut entries = tokio::fs::read_dir(directory).await?;

    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_dir() {
            match load_extension(&entry.path()).await {
                Ok(ext) => {
                    tracing::info!("Discovered extension: {}", ext.name);
                    extensions.push(ext);
                }
                Err(e) => {
                    tracing::warn!("Failed to load extension from {:?}: {}", entry.path(), e);
                }
            }
        }
    }

    Ok(extensions)
}

/// Extension loader for managing extension discovery
pub struct ExtensionLoader {
    extensions_dir: PathBuf,
}

impl ExtensionLoader {
    /// Create a new extension loader
    pub fn new(extensions_dir: PathBuf) -> Self {
        Self { extensions_dir }
    }

    /// Discover all extensions in the configured directory
    pub async fn discover_extensions(&self) -> Result<Vec<crate::ExtensionInfo>> {
        let extensions = discover_extensions(&self.extensions_dir).await?;

        // Convert to ExtensionInfo
        Ok(extensions
            .into_iter()
            .map(|ext| crate::ExtensionInfo {
                id: ext.id,
                name: ext.name,
                version: ext.version,
                path: ext.entry_point,
                capabilities: ext.capabilities,
            })
            .collect())
    }

    /// Get the extensions directory
    pub fn extensions_dir(&self) -> &Path {
        &self.extensions_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_capabilities() {
        let manifest = ExtensionManifest {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            display_name: None,
            description: None,
            publisher: None,
            main: None,
            activationEvents: None,
            contributes: Some(serde_json::json!({
                "languages": [{"id": "rust"}]
            })),
            engines: EngineRequirements {
                vscode: "^1.60.0".to_string(),
            },
        };

        let capabilities = parse_capabilities(&manifest);
        assert!(!capabilities.is_empty());
    }
}
