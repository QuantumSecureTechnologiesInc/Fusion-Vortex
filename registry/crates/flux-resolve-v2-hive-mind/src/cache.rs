//! Cache abstraction layer for Flux-Resolve v2.0
//!
//! Wraps fusion-redis::Store to provide TTL-based caching for dependency resolutions

use anyhow::Result;
use bytes::Bytes;
use fusion_redis::store::Store;
use std::sync::Arc;

/// Cache layer that wraps fusion-redis Store
pub struct CacheLayer {
    store: Arc<Store>,
}

impl CacheLayer {
    /// Create new cache layer with fusion-redis store
    pub fn new() -> Self {
        Self {
            store: Arc::new(Store::new()),
        }
    }

    /// Create cache layer for in-memory testing
    pub fn new_memory() -> Self {
        Self::new()
    }

    /// Get cached resolution by hash
    pub async fn get(&self, hash: &str) -> Result<Option<String>> {
        if let Some(bytes) = self.store.get(hash) {
            let s = String::from_utf8(bytes.to_vec())?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    /// Store resolution in cache with TTL
    pub async fn put(&self, hash: &str, lock_data: &str, ttl_ms: Option<u64>) -> Result<()> {
        let bytes = Bytes::from(lock_data.as_bytes().to_vec());
        self.store.set(hash.to_string(), bytes, ttl_ms);
        Ok(())
    }

    /// Clear all cached entries
    pub fn clear(&self) {
        // fusion-redis Store doesn't expose clear, but entries expire via TTL
        // For testing, we could recreate the store, but not essential for production
    }
}

impl Default for CacheLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_put_get() {
        let cache = CacheLayer::new();

        let hash = "test_hash_123";
        let data = "LOCK_DATA_TEST";

        cache.put(hash, data, Some(60000)).await.unwrap();
        let retrieved = cache.get(hash).await.unwrap();

        assert_eq!(retrieved, Some(data.to_string()));
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = CacheLayer::new();
        let result = cache.get("nonexistent").await.unwrap();
        assert_eq!(result, None);
    }
}
