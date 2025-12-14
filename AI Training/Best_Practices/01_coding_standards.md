# Fusion Best Practices - Expert Guide

**Dataset Category**: Best Practices  
**Training Level**: Intermediate to Expert  
**Last Updated**: December 2025 (v0.2.0-beta.1)  
**Source**: FUSION_COMPLETE_GUIDEBOOK.md

---

## Overview

This guide distills years of production Fusion experience into actionable best practices across all aspects of development, from language fundamentals to advanced features. Following these practices will help you write idiomatic, performant, and maintainable Fusion code.

## 1. Language Fundamentals Best Practices

### 1.1 Prefer Immutability

**Principle**: Use `let` (immutable) by default, `let mut` only when necessary.

```fusion
// ✅ Good: Immutable by default
let name = "Alice"
let age = 30
let scores = [95, 87, 92]

// ❌ Bad: Unnecessary mutability
let mut name = "Alice"  // Never modified
let mut age = 30        // Never modified
```

**Why**: Immutable data is easier to reason about, enables better compiler optimizations, and prevents accidental bugs.

### 1.2 Use GC by Default, @borrowed for Hot Paths

**Principle**: Start with garbage collection, optimize specific functions with `@borrowed`.

```fusion
// ✅ Good: GC for high-level logic
fn main() -> int {
    let data = load_large_dataset()
    let processed = process_data(&data)  // GC handles memory
    save_results(processed)
    return 0
}

// ✅ Good: @borrowed for critical path
@borrowed
fn process_audio_samples(buffer: &mut [f32]) {
    for sample in buffer {
        *sample = apply_filter(*sample)  // Zero allocations
    }
}

// ❌ Bad: Premature optimization
@borrowed  // Unnecessary for non-critical code
fn format_username(name: &str) -> String {
    format("User: {}", name)
}
```

**Why**: GC provides developer productivity. Reserve `@borrowed` for proven bottlenecks (profiling first).

### 1.3 Trust the Monolith

**Principle**: Keep `fusion watch` running for optimal development experience.

```bash
# ✅ Good: Start Monolith daemon
fusion watch &

# Now enjoy:
# - Instant type checking
# - Real-time security audits  
# - Sub-second builds
# - Zero-copy IDE integration
```

**Why**: The Monolith eliminates redundant parsing and provides near-instant feedback.

### 1.4 Annotate Public APIs with Explicit Types

**Principle**: Inference for local code, explicit types for public interfaces.

```fusion
// ✅ Good: Explicit public API
pub fn calculate_total(items: &[Item], tax_rate: f64) -> f64 {
    let subtotal = items.iter().map(|item| item.price).sum()
    subtotal * (1.0 + tax_rate)
}

// ✅ Good: Inference for internals
fn internal_helper() {
    let x = 42            // Type inferred
    let y = x * 2.0       // Type inferred
}

// ❌ Bad: Inference on public API
pub fn calculate_total(items, tax_rate) {  // Unclear types
    // Implementation
}
```

**Why**: Explicit types serve as documentation and prevent accidental API changes.

## 2. Flux-Resolve & Dependency Management

### 2.1 Enable Hive Mind for Team Projects

**Principle**: Share resolution cache across team via Redis.

```toml
# ✅ Good: Team configuration
[flux-resolve]
hive-mind = true
redis-url = "redis://cache.company.com:6379"
```

```bash
# Results:
# - 87%+ cache hit rate
# - Sub-second resolution for cached graphs
# - Shared security scan results
```

**Why**: Dramatically speeds up dependency resolution across team.

### 2.2 Monitor Cache Hit Rates

**Principle**: Track Hive Mind effectiveness and optimize.

```bash
# ✅ Good: Regular monitoring
fusion flux-resolve stats --hive-mind

# Target metrics:
# - Cache hit rate > 80%
# - Avg resolution time < 500ms
# - Security scan reuse > 90%
```

### 2.3 Use GPU Mode for Large Projects

**Principle**: Enable GPU acceleration for monorepos with complex dependency graphs.

```bash
# ✅ Good: GPU mode for 500+ dependencies
fusion flux-resolve --engine-mode gpu

# Speedups:
# - 500 deps: 29x faster
# - 1000 deps: 64x faster
# - 5000 deps: 95x faster
```

### 2.4 Always Keep Security Scanning Enabled

**Principle**: Never disable `--security-level strict` in production.

```toml
# ✅ Good: Strict security
[flux-resolve]
security-level = "strict"
fail-on-vulnerability = true

# ❌ Bad: Disabled security
[flux-resolve]
security-level = "permissive"  # Dangerous!
```

**Why**: Shift-left security prevents vulnerable dependencies from entering codebase.

### 2.5 Let Flux-Resolve Handle Version Conflicts

**Principle**: Avoid manual version overrides; trust the SAT solver.

```toml
# ✅ Good: Let Flux-Resolve resolve
[dependencies]
serde = "1.0"
tokio = "1.35"
# Flux-Resolve finds compatible versions

# ❌ Bad: Manual overrides
[dependencies.overrides]
serde = "=1.0.195"  // Brittle, breaks updates
```

**Why**: Flux-Resolve's SAT solver finds optimal compatible versions.

## 3. Runtime Core & Performance

### 3.1 Provide AI Scheduler Warm-Up Time

**Principle**: Set `FUSION_RUNTIME_WARMUP=true` and allow learning phase.

```bash
# ✅ Good: Enable warmup in development
export FUSION_RUNTIME_WARMUP=true
fusion run

# Save profile for production
fusion cortex save-profile production.cortex
```

**Why**: Cortex needs ~10,000 task completions to learn optimal scheduling.

### 3.2 Use HAL Annotations Liberally

**Principle**: Let `#[hal_accelerated]` choose the best execution device.

```fusion
// ✅ Good: HAL chooses CPU/GPU automatically
#[hal_accelerated]
fn matrix_operations(a: Tensor, b: Tensor) -> Tensor {
    a.matmul(b)  // Runs on GPU if available, CPU otherwise
}

// ❌ Bad: Manual device selection without reason
#[hal_accelerated(device = Device::GPU)]
fn simple_math(x: f32) -> f32 {
    x * 2.0  // Too simple for GPU, overhead outweighs benefit
}
```

**Why**: HAL makes intelligent decisions based on operation complexity and available hardware.

### 3.3 Profile Before Optimizing

**Principle**: Use `fusion profile record` to identify actual bottlenecks.

```bash
# ✅ Good: Profile-driven optimization
fusion profile record --release
fusion profile report

# Focus optimization on functions showing:
# - High wall time
# - High CPU time
# - Cache misses
```

**Why**: Intuition misleads; data reveals true bottlenecks.

### 3.4 Don't Force Device Selection

**Principle**: Let HAL auto-detect optimal devices unless you have specific requirements.

```fusion
// ✅ Good: Auto-detection
#[hal_accelerated]
fn ai_inference(input: Tensor) -> Tensor {
    model.forward(input)
}

// ⚠️ Only when necessary
#[hal_accelerated(device = Device::GPU)]
fn must_use_gpu(data: Tensor) -> Tensor {
    // Specific hardware requirement (e.g., customer requirement)
}
```

### 3.5 Avoid Ignoring Cortex Thrashing

**Principle**: If AI scheduler struggles, provide a saved profile or simplify workload.

```bash
# Symptoms: High CPU, poor scheduling
# Solution 1: Provide profile from testing
fusion cortex load-profile testing.cortex

# Solution 2: Reduce concurrent tasks
[runtime.cortex]
max_parallelism = 4  # Match available cores
```

## 4. HAFT & TensorWeave

### 4.1 Let HAFT Learn First

**Principle**: First run profiles access patterns; subsequent runs are optimized.

```fusion
// First run: HAFT learns
let tensor = FluxTensor::<f32>::random([10000, 10000])
for _ in 0..100 {
    process(tensor)  // HAFT observes access patterns
}

// Second run: HAFT applies optimizations
// - Memory layout optimized
// - Tiering strategy learned
// - Prefetching tuned
```

### 4.2 Save Profiles for Production

**Principle**: Capture learned HAFT profiles in development, load in production.

```bash
# Development: Let HAFT learn
fusion run --profile-haft
fusion haft save-profile production.haft

# Production: Skip learning phase
export FUSION_HAFT_PROFILE=production.haft
fusion run --release
```

**Why**: Eliminates warmup overhead in production.

### 4.3 Use TensorWeave for  Multi-Tensor Workflows

**Principle**: Graph optimization provides significant benefits for complex pipelines.

```fusion
use fusion::tensorweave::Graph

// ✅ Good: Graph optimization for complex workflow
let graph = Graph::new()
    .add_op(normalize_op)
    .add_op(transform_op)
    .add_op(aggregate_op)
    .optimize()  // Fuses ops, minimizes data movement

let result = graph.execute(input_tensor)?

// ❌ Bad: Manual orchestration
let normalized = normalize(input)
let transformed = transform(normalized)  // Unnecessary copy
let result = aggregate(transformed)      // Another copy
```

### 4.4 Don't Micromanage Memory Tiers

**Principle**: Trust the Builder Agent to tier data optimally.

```fusion
// ✅ Good: Let HAFT manage tiers
let dataset = FluxTensor::<f32>::from_file("100GB_data.parquet")
for batch in dataset.batches(32) {
    train_model(batch)  // HAFT handles GPU/CPU/NVMe automatically
}

// ❌ Bad: Manual tier management (usually unnecessary)
dataset.pin_to_tier(Tier::Hot)  // Override Builder Agent
```

### 4.5 Cache Optimized Graphs

**Principle**: For training loops, cache graphs with `graph.save()`.

```fusion
// ✅ Good: Cache training graph
let graph = build_training_graph()
    .optimize()
    .save("training_graph.twe")?

// Subsequent runs: instant startup
let graph = Graph::load("training_graph.twe")?

for epoch in 0..100 {
    graph.execute(training_data)?
}
```

## 5. Security with Sentinel TriBrid

### 5.1 Enable Full TriBrid Mode

**Principle**: All three subsystems (Chaos, Mesh, Adaptive) complement each other.

```fusion
// ✅ Good: Full protection
#[tribrid_protected]
mod sensitive_api {
    // Gets:
    // - Chaos encryption
    // - Credential rotation
    // - Anomaly detection
}

// ❌ Bad: Partial protection
#[tribrid_protected(
    chaos = false,  // Weaker encryption
    mesh = false    // No rotation 
)]
mod half_protected {
    // Vulnerable!
}
```

### 5.2 Configure Appropriate Rotation Periods

**Principle**: Balance security vs. complexity.

```toml
# ✅ Good: Environment-appropriate rotation
[sentinel.mesh]
# High-security: Finance, healthcare
rotation_period_secs = 10

# Standard: Business applications
rotation_period_secs = 60

# Low-security: Public APIs
rotation_period_secs = 300
```

### 5.3 Provide Adequate Warmup Data

**Principle**: Adaptive Threat Response needs ≥10,000 samples for accurate modeling.

```toml
[sentinel.adaptive]
warmup_samples = 10000  # Minimum
warmup_samples = 50000  # Better for complex apps
```

### 5.4 Never Disable Auto-Response in Production

**Principle**: Sentinel's automated threat response prevents attacks in real-time.

```toml
# ✅ Good: Automated response
[sentinel.adaptive]
auto_response = true

# ❌ Bad: Manual-only (too slow)
[sentinel.adaptive]
auto_response = false  // Dangerous!
```

### 5.5 Avoid Overly Short Rotation Periods

**Principle**: <5 seconds can cause legitimate request failures.

```toml
# ❌ Bad: Too aggressive
[sentinel.mesh]
rotation_period_secs = 2  // Causes failures

# ✅ Good: Balanced
[sentinel.mesh]
rotation_period_secs = 10  // Secure + stable
overlap_period_secs = 5    // Graceful transition
```

## 6. Testing Strategies

### 6.1 Write Unit Tests for Business Logic

```fusion
#[test]
fn test_calculate_discount() {
    let items = vec![
        Item { price: 100.0 },
        Item { price: 50.0 }
    ]
    assert_eq!(calculate_discount(&items), 15.0)
}
```

### 6.2 Use Integration Tests for Multi-Component Features

```fusion
#[test]
async fn test_payment_flow() {
    let db = test_database().await
    let api = test_api_server().await
    
    let response = api.post("/payment")
        .json(payment_request)
        .send().await?
    
    assert_eq!(response.status(), 200)
    assert!(db.payment_exists(payment_id).await)
}
```

### 6.3 Benchmark Performance-Critical Code

```fusion
use fusion::bench::Bencher

#[bench]
fn bench_sorting(b: &mut Bencher) {
    let data = generate_test_data(10000)
    b.iter(|| {
        quicksort(&mut data.clone())
    })
}
```

## 7. Error Handling

### 7.1 Use Result for Recoverable Errors

```fusion
// ✅ Good: Result for recoverable errors
fn parse_config(path: &str) -> Result<Config, ConfigError> {
    let contents = fs::read_to_string(path)?
    let config: Config = toml::from_str(&contents)?
    Ok(config)
}

// ❌ Bad: panic for recoverable errors
fn parse_config(path: &str) -> Config {
    let contents = fs::read_to_string(path).unwrap()  // Crashes!
    toml::from_str(&contents).unwrap()  // Crashes!
}
```

### 7.2 Panic Only for Unrecoverable Errors

```fusion
// ✅ Good: panic for programmer errors
fn get_element(index: usize, data: &[int]) -> int {
    assert!(index < data.len(), "Index out of bounds")
    data[index]
}

// ❌ Bad: Result for programmer errors
fn get_element(index: usize, data: &[int]) -> Result<int> {
    // Caller shouldn't handle this; it's a bug
}
```

### 7.3 Provide Context with Error Types

```fusion
// ✅ Good: Rich error types
enum ConfigError {
    FileNotFound(PathBuf),
    ParseError { line: usize, message: String },
    ValidationError(String)
}

// ❌Bad: String errors
fn load_config() -> Result<Config, String> {
    // Lost type information
}
```

---

## Quick Reference Checklist

### Daily Development
- [ ] Keep `fusion watch` running
- [ ] Run `fusion fmt` before committing
- [ ] Run `fusion clippy` to catch issues
- [ ] Run `fusion test` for changed modules

### Before Committing
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted
- [ ] Security audit clean (`fusion audit`)

### Before Releasing
- [ ] Integration tests pass
- [ ] Benchmarks show no regressions
- [ ] Security audit strict mode passes
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

### Production Deployment
- [ ] Cortex profile saved and loaded
- [ ] HAFT profile saved and loaded
- [ ] Security level = strict
- [ ] Monitoring configured
- [ ] Backup/rollback plan ready

---

## Key Takeaways for AI Training

1. **Immutability First**: Use `let` by default, `let mut` only when needed
2. **GC by Default**: Optimize with `@borrowed` only for proven bottlenecks
3. **Trust the Tools**: Monolith, Flux-Resolve, HAL make intelligent decisions
4. **Profile-Driven**: Measure before optimizing
5. **Hive Mind**: Essential for team productivity
6. **Security Always On**: Never disable strict mode in production
7. **Let AI Learn**: Cortex, HAFT, Sentinel need warmup time
8. **Save Profiles**: Development learns, production loads
9. **Result for Errors**: Recoverable errors, panic for bugs
10. **Test Everything**: Unit + integration + benchmarks

These best practices represent collective wisdom from thousands of hours of production Fusion development. Following them will help you write idiomatic, performant, and maintainable code. Cross-reference with troubleshooting, performance optimization, and security datasets for comprehensive guidance.
