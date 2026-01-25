# Fusion Crate Archetype Quick Reference

## At a Glance

| Type             | Label           | Features       | Docs Focus  | Example               |
| ---------------- | --------------- | -------------- | ----------- | --------------------- |
| **Primitive**    | `Foundation:`   | None default   | Panics, API | `fusion_std`          |
| **Algorithm**    | `Algorithm:`    | Minimal        | Complexity  | `clustering`          |
| **Integration**  | `Integration:`  | async/blocking | Setup       | `fusion_http`         |
| **Framework**    | `Framework:`    | full/modular   | Guides      | `fusion_runtime_core` |
| **Tooling**      | `Tool:`         | Full DX        | CLI usage   | `fusion` CLI          |
| **Experimental** | `EXPERIMENTAL:` | Research       | Roadmap     | `flux-resolve-v2`     |

## Decision Tree

```text
Is it copyable to other projects?
├─ YES → Foundation/Primitive
└─ NO
    └─ Does it solve a specific algorithm problem?
        ├─ YES → Algorithm/Engine
        └─ NO
            └─ Does it connect to external services?
                ├─ YES → Integration/Glue
                └─ NO
                    └─ Does it provide structure/architecture?
                        ├─ YES → Framework
                        └─ NO
                            └─ Is it a command-line tool?
                                ├─ YES → Tooling
                                └─ NO → Experimental
```text

## Fusion.toml Patterns

### Foundation

```toml
description = "Foundation: {brief}. Panic-free."
keywords = ["{domain}", "primitives"]
categories = ["development-tools"]
[features]
default = []  # No defaults!
```text

### Algorithm

```toml
description = "Algorithm: {name}. O({complexity})."
keywords = ["{domain}", "algorithm"]
categories = ["algorithms", "science"]

# + benchmarks required

```text

### Integration

```toml
description = "Integration: {service} connector"
keywords = ["{service}", "integration"]
[features]
default = ["async"]
async = ["tokio"]
blocking = []
```text

### Framework

```toml
description = "Framework: Opinionated {domain}"
keywords = ["framework", "{domain}"]
[features]
default = ["full"]
full = ["comp1", "comp2"]
```text

### Tooling

```toml
description = "Tool: {purpose} with excellent errors"
keywords = ["tool", "cli", "{domain}"]
[[bin]]
name = "{tool}"
```text

### Experimental

```toml
version = "0.1.0"  # <1.0!
description = "EXPERIMENTAL: {research}. Not production."
keywords = ["experimental", "research"]
```text

## Code Patterns

### Primitive

```rust
#![forbid(unsafe_code)]  // If possible

/// Returns result. Never panics on valid input.
///
/// # Panics
/// - If input.len() == 0
pub fn process(input: &[u8]) -> Result<T> { }
```text

### Algorithm

```rust
/// K-Means clustering.
///
/// # Complexity
/// - Time: O(n·k·i) where n=samples, k=clusters, i=iterations
/// - Space: O(n + k)
///
/// # When NOT to Use
/// - Categorical data (use DBSCAN)
/// - Unknown k (use hierarchical)
pub struct KMeans { }
```text

### Integration

```rust

#[cfg(feature = "async")]

pub async fn fetch(url: &str) -> Result<T> { }

#[cfg(feature = "blocking")]

pub fn fetch_blocking(url: &str) -> Result<T> { }
```text

### Framework

```rust
/// Core runtime for hybrid workloads.
///
/// # Quick Start
/// ```
/// let runtime = Runtime::new();
/// runtime.spawn(task);
/// ```
///
/// See guides/ for detailed documentation.
pub struct Runtime { }
```text

### Tooling

```rust
// Human-readable by default
println!("✓ Build successful");

// Machine-readable on demand
if args.json {
    println!(r#"{{"status":"success"}}"#);
}
```text

## Documentation Structure

### Primitive README

```markdown

# {crate}

{one-liner}

## What

{single purpose}

## When

{use cases}

## API

{minimal examples}

## Panics

{explicit list}
```text

### Algorithm README

```markdown

# {algorithm}

{problem solved}

## Complexity

- Time: O(...)
- Space: O(...)

## When to Use

{scenarios}

## When NOT to Use

{alternatives}

## Benchmarks

{link to benches/}
```text

### Integration README

```markdown

# {service} Integration

## Quick Start

{5-line example}

## Features

| Feature  | Description |
| -------- | ----------- |
| async    | {desc}      |
| blocking | {desc}      |

## Common Pitfalls

{list}
```text

### Framework README

```markdown

# {Framework}

## Getting Started

{tutorial}

## Core Concepts

{architecture}

## Advanced

{customization}

## Extending

{plugin system}
```text

### Tool README

```markdown

# {Tool}

## Installation

{one command}

## Usage

{common commands}

## Configuration

{config file}

## Exit Codes

{list}
```text

## Quality Checklist

### All Crates

- [ ] Description starts with archetype label
- [ ] Keywords include archetype
- [ ] Categories appropriate
- [ ] README.md exists

### Primitive

- [ ] No default features
- [ ] Minimal dependencies
- [ ] All panics documented
- [ ] Benchmarks present

### Algorithm

- [ ] Complexity in description
- [ ] Benchmarks present
- [ ] "When NOT" section
- [ ] Deterministic noted

### Integration

- [ ] Feature flags
- [ ] async/blocking split
- [ ] Setup examples
- [ ] Pitfalls section

### Framework

- [ ] Full vs modular features
- [ ] Guide-first docs
- [ ] Migration guide
- [ ] Examples directory

### Tooling

- [ ] CLI help text
- [ ] Error messages
- [ ] --json support
- [ ] Exit codes

### Experimental

- [ ] Version <1.0
- [ ] EXPERIMENTAL warning
- [ ] Roadmap present
- [ ] Limitations listed

## Anti-Patterns

### DON'T: One-Size-Fits-All

```toml

# ❌ Same approach for all crates

[features]
default = ["everything"]
```text

### DO: Archetype-Specific

```toml

# ✅ Primitive: no defaults

[features]
default = []

# ✅ Framework: full defaults

[features]
default = ["full"]
```text

### DON'T: Vague Descriptions

```toml

# ❌ What does it do?

description = "Useful utilities"
```text

### DO: Clear Purpose

```toml

# ✅ Archetype + purpose

description = "Foundation: Error handling primitives"
```text

### DON'T: Hidden Complexity

```rust
// ❌ Users need to know this!
pub fn sort(data: &mut [i32]) {
    // Actually O(n²) bubble sort
}
```text

### DO: Explicit Guarantees

```rust
/// Sorts using merge sort.
///
/// # Complexity
/// - Best/Average/Worst: O(n log n)
/// - Space: O(n)
pub fn sort(data: &mut [i32]) { }
```text

## Migration Guide

### Upgrading a Primitive

1. Remove default features
2. Make serde optional
3. Add panic documentation
4. Add benchmarks
5. Update description with "Foundation:"

### Upgrading an Algorithm

1. Document complexity
2. Add "When NOT" section
3. Include benchmarks
4. Verify determinism
5. Update description with "Algorithm:" + O(n)

### Upgrading Integration

1. Split async/blocking features
2. Set sensible default
3. Add feature matrix
4. Document pitfalls
5. Update description with "Integration:"

### Upgrading Framework

1. Create full/modular features
2. Structure guides (Start/Concepts/Advanced/Extend)
3. Add migration guide for breaks
4. Update description with "Framework:"

### Upgrading Tool

1. Ensure human-readable default
2. Add --json for machines
3. Document exit codes
4. Polish error messages
5. Update description with "Tool:"

### Marking Experimental

1. Set version to 0.x.x
2. Prefix description with "EXPERIMENTAL:"
3. Add prominent warning
4. Include roadmap
5. List limitations

---

**Print this guide** for quick reference during development.