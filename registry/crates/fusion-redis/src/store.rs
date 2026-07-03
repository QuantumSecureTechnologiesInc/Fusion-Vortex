use bytes::Bytes;
#[cfg(feature = "persistence")]
use dashmap::DashMap;
#[cfg(not(feature = "persistence"))]
use std::collections::HashMap;
#[cfg(not(feature = "persistence"))]
use std::sync::RwLock;
use std::time::Instant;

#[derive(Clone)]
pub struct Entry {
    pub value: Bytes,
    pub expires_at: Option<Instant>,
}

impl Entry {
    pub fn new(value: Bytes, ttl_ms: Option<u64>) -> Self {
        let expires_at = ttl_ms.map(|ms| Instant::now() + std::time::Duration::from_millis(ms));
        Self { value, expires_at }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(at) => Instant::now() > at,
            None => false,
        }
    }
}

pub struct Store {
    #[cfg(feature = "persistence")]
    data: DashMap<String, Entry>,
    #[cfg(not(feature = "persistence"))]
    data: RwLock<HashMap<String, Entry>>,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }

    pub fn set(&self, key: String, value: Bytes, ttl_ms: Option<u64>) {
        self.data.insert(key, Entry::new(value, ttl_ms));
    }

    pub fn get(&self, key: &str) -> Option<Bytes> {
        // We use a closure to avoid deadlocks or holding locks too long,
        // though DashMap handles shard locking.
        // We need to check expiration.

        // This is a bit tricky with DashMap because we can't easily remove while holding a reference in some versions,
        // but DashMap specifically allows upgrading or we can just remove after checking.
        if let Some(entry) = self.data.get(key) {
            if entry.is_expired() {
                drop(entry); // Release read lock
                self.data.remove(key); // Remove it
                return None;
            }
            return Some(entry.value.clone());
        }
        None
    }

    pub fn del(&self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    pub fn exists(&self, key: &str) -> bool {
        if let Some(entry) = self.data.get(key) {
            if entry.is_expired() {
                drop(entry);
                self.data.remove(key);
                return false;
            }
            return true;
        }
        false
    }
}
