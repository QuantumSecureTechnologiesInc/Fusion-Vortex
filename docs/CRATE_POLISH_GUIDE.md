# Fusion Crate Polish Automation Script

This document defines the systematic updates to be applied to each crate category.

## Template Patterns by Archetype

### 1. Foundation/Primitive Pattern

```toml
[package]
name = "{crate_name}"
version = "0.2.0"
edition = "2021"
license = "MIT"
description = "Foundation: {brief_description}. {panic_behavior}"
authors = ["Fusion Team"]
repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
keywords = ["{keyword1}", "{keyword2}", "primitives"]
categories = ["development-tools"]
readme = "README.md"

[features]
default = []

[dependencies]

# Minimal dependencies only

[dev-dependencies]
criterion = { workspace = true }

[[bench]]
name = "{crate_name}_bench"
harness = false
```text

**Source code requirements**:
- Add `#![forbid(unsafe_code)]` where possible
- Document all panics explicitly
- No logging statements
- No async
- No global state

### 2. Algorithm/Engine Pattern

```toml
[package]
name = "{crate_name}"
version = "0.2.0"
edition = "2021"
license = "MIT"
description = "Algorithm: {algorithm_name}. {complexity_guarantee}"
authors = ["Fusion Team"]
repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
keywords = ["{domain}", "algorithm", "fusion"]
categories = ["algorithms", "science"]
readme = "README.md"

[dependencies]
fusion_core = { workspace = true }

# Algorithm-specific deps

[dev-dependencies]
criterion = { workspace = true }

[[bench]]
name = "{algorithm}_bench"
harness = false
```text

**Documentation requirements**:
- Big-O complexity in description
- "When NOT to use" section
- Performance characteristics
- Deterministic behavior guarantees

### 3. Integration/Glue Pattern

```toml
[package]
name = "{crate_name}"
version = "0.2.0"
edition = "2021"
license = "MIT"
description = "Integration: {service/protocol} connector with {key_features}"
authors = ["Fusion Team"]
repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
keywords = ["{service}", "integration", "connector"]
categories = ["network-programming"]
readme = "README.md"

[features]
default = ["async"]
async = ["tokio"]
blocking = []
tls = ["{tls_dep}"]

[dependencies]
fusion_runtime_core = { workspace = true }
tokio = { workspace = true, optional = true }

# Service-specific dependencies

[dev-dependencies]
tokio = { workspace = true, features = ["macros", "rt-multi-thread"] }
```text

**Documentation requirements**:
- Feature matrix table
- Setup examples first
- Common pitfalls section
- Async/blocking alternatives

### 4. Framework Pattern

```toml
[package]
name = "{crate_name}"
version = "0.2.0"
edition = "2021"
license = "MIT"
description = "Framework: {framework_purpose} - opinionated {domain} framework"
authors = ["Fusion Team"]
repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
keywords = ["{domain}", "framework", "runtime"]
categories = ["development-tools::build-utils"]
readme = "README.md"

[features]
default = ["full"]
full = ["runtime", "scheduler", "memory"]
runtime = []
scheduler = []
memory = []

[dependencies]
fusion_core = { workspace = true }

# Framework dependencies

[dev-dependencies]

# Framework test dependencies

```text

**Documentation requirements**:
- Getting Started guide
- Core Concepts section
- Advanced Usage section
- Extending the Framework section
- Migration guide (for breaking changes)

### 5. Tooling Pattern

```toml
[package]
name = "{crate_name}"
version = "0.2.0"
edition = "2021"
license = "MIT"
description = "Tool: {tool_purpose} with excellent error messages and CI-friendly output"
authors = ["Fusion Team"]
repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
keywords = ["{domain}", "tool", "cli"]
categories = ["development-tools"]
readme = "README.md"

[[bin]]
name = "{tool_name}"
path = "src/main.rs"

[dependencies]
clap = { workspace = true }
anyhow = { workspace = true }

# Tool-specific deps

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
```text

**CLI requirements**:
- Human-readable output by default
- `--json` flag for machine-readable
- Colored output (respects NO_COLOR)
- Progress indicators
- Exit codes documented

### 6. Experimental Pattern

```toml
[package]
name = "{crate_name}"
version = "0.1.0"  # Note: <1.0 for experimental
edition = "2021"
license = "MIT"
description = "EXPERIMENTAL: {research_area}. Not production-hardened."
authors = ["Fusion Team"]
repository = "https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language"
keywords = ["{domain}", "experimental", "research"]
categories = ["science"]
readme = "README.md"

[dependencies]
fusion_core = { workspace = true }

# Research dependencies

[package.metadata.docs.rs]
rustdoc-args = ["--html-in-header", "experimental-warning.html"]
```text

**Documentation requirements**:
- Prominent EXPERIMENTAL warning
- Clear roadmap
- Known limitations section
- Research paper citations (if applicable)

## Batch Update Priority

1. **Phase 1: Core Primitives** (15 crates)
   - std, fusion_core, fusion_std
   - Math primitives
   - Crypto primitives

2. **Phase 2: Key Algorithms** (20 crates)
   - clustering, attention, resnet
   - Quantum algorithms
   - ML/AI engines

3. **Phase 3: Integration Points** (25 crates)
   - HTTP, gRPC, database connectors
   - Cloud integrations
   - Interop bridges

4. **Phase 4: Frameworks** (15 crates)
   - Runtime components
   - AI frameworks
   - Service frameworks

5. **Phase 5: Tooling** (20 crates)
   - CLI tools
   - Analysis tools
   - Security tools

6. **Phase 6: Experimental** (10 crates)
   - Research projects
   - Novel algorithms
   - Early prototypes

## Automated Checks

For each crate, verify:
- [ ] Description starts with category label
- [ ] Keywords include category
- [ ] README.md exists
- [ ] Features are properly documented
- [ ] Benchmarks for algorithms/primitives
- [ ] Examples in crate root
- [ ] CHANGELOG.md for frameworks/tools