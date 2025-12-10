// src/registry/api.rs - REST API for Package Registry
#![allow(dead_code)]
// HTTP server endpoints for package operations

use super::{PackageMetadata, RegistryError};
use std::collections::HashMap;

/// API endpoint handlers
pub struct ApiServer {
    /// Port to listen on
    port: u16,
    /// Package store
    packages: HashMap<String, Vec<PackageMetadata>>,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            packages: HashMap::new(),
        }
    }

    /// Start the API server
    pub fn start(&self) -> Result<(), RegistryError> {
        println!("Registry API server starting on port {}", self.port);
        Ok(())
    }

    /// Handle package upload
    pub fn upload_package(
        &mut self,
        metadata: PackageMetadata,
        _data: Vec<u8>,
    ) -> Result<(), RegistryError> {
        self.packages
            .entry(metadata.name.clone())
            .or_insert_with(Vec::new)
            .push(metadata);
        Ok(())
    }

    /// Handle package download
    pub fn download_package(&self, name: &str, version: &str) -> Result<Vec<u8>, RegistryError> {
        if let Some(versions) = self.packages.get(name) {
            if versions.iter().any(|v| v.version == version) {
                return Ok(vec![]); // Placeholder
            }
        }
        Err(RegistryError::PackageNotFound(name.to_string()))
    }

    /// Search packages
    pub fn search(&self, query: &str) -> Vec<&PackageMetadata> {
        self.packages
            .values()
            .flatten()
            .filter(|p| p.name.contains(query) || p.description.contains(query))
            .collect()
    }
}
