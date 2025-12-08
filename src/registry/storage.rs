// src/registry/storage.rs - Storage backend implementation
#![allow(dead_code)]

use super::{RegistryError, StorageBackend};
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Storage {
    backend: StorageBackend,
    path: PathBuf,
}

impl Storage {
    pub fn new(backend: StorageBackend, path: PathBuf) -> Self {
        Self { backend, path }
    }

    pub fn store(&self, _key: &str, _data: &[u8]) -> Result<(), RegistryError> {
        Ok(())
    }

    pub fn retrieve(&self, _key: &str) -> Result<Vec<u8>, RegistryError> {
        Ok(vec![])
    }

    pub fn delete(&self, _key: &str) -> Result<(), RegistryError> {
        Ok(())
    }
}
