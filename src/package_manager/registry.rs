#![allow(dead_code)]
#![allow(unused_variables)]

use super::{Package, Version};

/// Registry client for package queries
pub struct RegistryClient {
    registry_url: String,
}

impl RegistryClient {
    pub fn new(registry_url: String) -> Self {
        RegistryClient { registry_url }
    }

    /// Get package metadata from registry
    pub fn get_metadata(&mut self, name: &str) -> Result<PackageMetadata, String> {
        // TODO: Make HTTP request to registry
        // For now, return mock data

        Ok(PackageMetadata {
            name: name.to_string(),
            description: "Package from registry".to_string(),
            latest_version: Version::new(1, 0, 0),
            versions: vec![Version::new(1, 0, 0)],
            downloads: 0,
            repository: None,
        })
    }

    /// Search packages in registry
    pub fn search(&mut self, query: &str) -> Result<Vec<PackageMetadata>, String> {
        // TODO: Implement registry search
        println!("Searching for packages matching '{}'...", query);
        Ok(vec![])
    }

    /// Publish package to registry
    pub fn publish(&mut self, package: &Package, tarball_path: &str) -> Result<(), String> {
        // TODO: Upload package tarball to registry
        println!(
            "Publishing {} {}...",
            package.name,
            package.version.to_string()
        );
        Ok(())
    }
}

/// Package metadata from registry
#[derive(Debug, Clone)]
pub struct PackageMetadata {
    pub name: String,
    pub description: String,
    pub latest_version: Version,
    pub versions: Vec<Version>,
    pub downloads: u64,
    pub repository: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_client() {
        let mut client = RegistryClient::new("https://packages.fusion-lang.org".to_string());
        let result = client.get_metadata("test-package");
        assert!(result.is_ok());
    }
}
