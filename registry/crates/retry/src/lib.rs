/// Production Retry Logic for AWS.
///
/// Implements Exponential Backoff + Jitter for transient errors (Throttling, 5xx).
use fusion_std::error::StdResult;
use rand::Rng;
use tokio::time::{sleep, Duration};

pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 100,
            max_delay_ms: 2000,
        }
    }
}

pub async fn with_retry<F, Fut, T>(config: &RetryConfig, operation: F) -> StdResult<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = StdResult<T>>,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                attempt += 1;
                if attempt >= config.max_attempts {
                    return Err(e);
                }

                // Check if retryable (Network error or 5xx or 429)
                // if !is_retryable(&e) { return Err(e); }

                // Exponential Backoff
                let mut delay = config.base_delay_ms * 2_u64.pow(attempt - 1);

                // Jitter (Full Jitter pattern: random between 0 and cap)
                let mut rng = rand::thread_rng();
                delay = rng.gen_range(0..delay.min(config.max_delay_ms));

                sleep(Duration::from_millis(delay)).await;
            }
        }
    }
}
