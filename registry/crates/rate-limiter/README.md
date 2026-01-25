# Rate Limiter

A token bucket rate limiter for controlling resource access in Fusion applications.

## Features

- Distributed rate limiting (optional Redis backend)
- Async support
- Configurable quotas

## Usage

```rust
use rate_limiter::RateLimiter;

let limiter = RateLimiter::new(100, 1.0); // 100 tokens, refill 1/sec
```text