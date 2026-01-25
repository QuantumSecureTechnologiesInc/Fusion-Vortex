// __FU_COMPAT_START__
#![allow(missing_docs)]
use std::collections::HashMap;
use std::time::{Duration, Instant};
#[allow(missing_docs, dead_code)] type FString = String;
#[allow(missing_docs, dead_code)] type FMap<K, V> = HashMap<K, V>;
// __FU_COMPAT_END__
/// Cache entry
#[derive(Clone)]
pub struct CacheEntry<T> {
    value: T,
    inserted_at: Instant,
    ttl: Duration,
}
/// Simple in-memory cache with TTL
pub struct Cache<T: Clone> {
    entries: FMap<FString, CacheEntry<T>>,
}
impl<T: Clone> Cache<T> {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }
    pub fn insert(&mut self, key: FString, value: T, ttl: Duration) {
        self.entries
            .insert(
                key,
                CacheEntry {
                    value,
                    inserted_at: Instant::now(),
                    ttl,
                },
            );
    }
    pub fn get(&mut self, key: &str) -> Option<T> {
        if let Some(entry) = self.entries.get(key) {
            if entry.inserted_at.elapsed() < entry.ttl {
                return Some(entry.value.clone());
            } else {
                self.entries.remove(key);
            }
        }
        None
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
impl<T: Clone> Default for Cache<T> {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_cache_insert_and_get() {
        let mut cache = Cache::new();
        cache.insert("key1".to_string(), "value1".to_string(), Duration::from_secs(10));
        let value = cache.get("key1");
        assert_eq!(value, Some("value1".to_string()));
    }
    #[test]
    fn test_cache_ttl() {
        let mut cache = Cache::new();
        cache
            .insert("key1".to_string(), "value1".to_string(), Duration::from_millis(50));
        thread::sleep(Duration::from_millis(100));
        let value = cache.get("key1");
        assert_eq!(value, None);
    }
}
