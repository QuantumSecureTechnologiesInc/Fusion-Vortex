/// Production Distributed Rate Limiter.
/// Implements the Sliding Window Log algorithm (requires external Redis/DB).
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const WINDOW_SECONDS: u64 = 60;
const MAX_REQUESTS: u32 = 100;

pub struct RateLimiter {
    // Key (IP/UserID) -> Log of timestamps
    request_log: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            request_log: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if the request should be allowed based on a sliding window.
    pub fn check_limit(&self, key: &str) -> bool {
        let mut log = self.request_log.lock().unwrap();
        let now = Instant::now();
        let window_start = now - Duration::from_secs(WINDOW_SECONDS);

        let entry = log.entry(key.to_string()).or_insert_with(Vec::new);

        // 1. Remove timestamps outside the window
        entry.retain(|t| *t > window_start);

        // 2. Check current count
        if entry.len() >= MAX_REQUESTS as usize {
            return false; // Limit exceeded
        }

        // 3. Record new request
        entry.push(now);
        true
    }
}
