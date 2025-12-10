#![allow(dead_code)]
#![allow(unused_variables)]

use super::{Dependency, Package, Version};
use std::fs;
use std::path::Path;

/// Package manifest (fusion.toml)
#[derive(Debug, Clone)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub edition: Option<String>,
    pub description: Option<String>,
    pub license: Option<String>,
}

impl Manifest {
    /// Load manifest from fusion.toml
    pub fn load(path: &Path) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read manifest: {}", e))?;

        // TODO: Parse TOML properly using serde
        // For now, return a default manifest
        Ok(Manifest {
            package: PackageInfo {
                name: "example".to_string(),
                version: "0.1.0".to_string(),
                authors: vec!["Author <author@example.com>".to_string()],
                edition: Some("2024".to_string()),
                description: Some("Example package".to_string()),
                license: Some("MIT".to_string()),
            },
            dependencies: vec![],
            dev_dependencies: vec![],
        })
    }

    /// Generate a manifest file
    pub fn generate(name: &str, version: &str) -> Result<String, String> {
        let template = format!(
            r#"[package]
name = "{}"
version = "{}"
authors = ["Your Name <you@example.com>"]
edition = "2024"

[dependencies]
# Add dependencies here
# collections = "1.0"

[dev-dependencies]
# Add dev dependencies here
# test-framework = "1.0"
"#,
            name, version
        );

        Ok(template)
    }

    /// Convert to Package
    pub fn to_package(&self) -> Package {
        // Parse version string to Version type
        let version =
            Version::parse(&self.package.version).unwrap_or_else(|_| Version::new(0, 1, 0));

        Package {
            name: self.package.name.clone(),
            version,
            authors: self.package.authors.clone(),
            description: self.package.description.clone(),
            license: self.package.license.clone(),
            repository: None,
            dependencies: self.dependencies.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_manifest() {
        let manifest = Manifest::generate("my-project", "0.1.0").unwrap();
        assert!(manifest.contains("[package]"));
        assert!(manifest.contains("my-project"));
    }
}
