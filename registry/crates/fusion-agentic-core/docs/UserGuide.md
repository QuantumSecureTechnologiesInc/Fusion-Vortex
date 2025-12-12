# Fusion Agentic Core - User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Core Concepts](#core-concepts)
5. [Feature Guide](#feature-guide)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

## Introduction

Welcome to the Fusion Agentic Core! This crate provides AI-enhanced capabilities for in-depth agentic reasoning, vibe coding, chain-of-thought processing, and code excellence enforcement.

### What is Agentic Reasoning?

Agentic reasoning is a multi-layer problem-solving approach that mimics how expert developers think:
- **Decomposition**: Breaking complex problems into manageable parts
- **Exploration**: Generating multiple solution approaches
- **Evaluation**: Selecting the best solution based on confidence
- **Reflection**: Self-critically evaluating and iterating

### What is Vibe Coding?

Vibe coding is pattern-based intuitive code generation that translates natural language intent into code using recognised patterns and templates.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
fusion-agentic-core = "0.1.0"
```

Then run:

```bash
cargo build
```

## Getting Started

### Basic Usage

```rust
use fusion_agentic_core::{AgenticCore, Result};

fn main() -> Result<()> {
    // Create the core
    let core = AgenticCore::new();
    
    // Solve a problem
    let solution = core.process_problem("Your problem here")?;
    println!("{}", solution);
    
    Ok(())
}
```

### Quick Examples

#### Problem Solving

```rust
let core = AgenticCore::new();
let solution = core.process_problem(
    "Design a caching system for a distributed application"
)?;
```

#### Code Generation

```rust
let core = AgenticCore::new();
let code = core.vibe_code(
    "filter even numbers and double them"
)?;
```

#### Quality Analysis

```rust
let core = AgenticCore::new();
let metrics = core.check_excellence(source_code)?;

if metrics.is_excellent() {
    println!("Excellent code quality!");
}
```

## Core Concepts

### AgenticCore

The main entry point that integrates all modules:

```rust
pub struct AgenticCore {
    agentic_engine: Arc<RwLock<AgenticEngine>>,
    chain_processor: Arc<ChainOfThought>,
    vibe_engine: Arc<VibeEngine>,
    excellence_enforcer: Arc<ExcellenceEnforcer>,
}
```

### Agentic Context

Represents the problem-solving state:

```rust
pub struct AgenticContext {
    pub problem: String,
    pub understanding: f64,  // 0.0 - 1.0
    pub confidence: f64,     // 0.0 - 1.0
    pub metadata: HashMap<String, String>,
    pub history: Vec<ReasoningStep>,
}
```

### Thought Nodes

Building blocks of chain-of-thought processing:

```rust
pub struct ThoughtNode {
    pub content: String,
    pub parent: Option<Box<ThoughtNode>>,
    pub children: Vec<ThoughtNode>,
    pub depth: usize,
    pub confidence: f64,
    pub tags: Vec<String>,
}
```

### Code Patterns

Templates for vibe coding:

```rust
pub struct CodePattern {
    pub name: String,
    pub category: PatternCategory,
    pub template: String,
    pub variables: Vec<String>,
    pub confidence: f64,
    pub examples: Vec<String>,
}
```

## Feature Guide

### 1. Agentic Reasoning

The agentic engine performs multi-layer reasoning:

```rust
use fusion_agentic_core::{AgenticEngine, AgenticContext};

let mut engine = AgenticEngine::new();
let context = AgenticContext::new("Problem description".to_string());

// This will iterate up to 10 times or until confidence >= 0.8
let solution = engine.reason(&context, &chain)?;
```

**Key Features:**
- Self-reflection loop
- Confidence tracking
- Adaptive iteration
- Context preservation

### 2. Chain of Thought

Structure your reasoning process:

```rust
use fusion_agentic_core::ChainOfThought;

let cot = ChainOfThought::new();

// Decompose a problem
let chain = cot.decompose("Implement a rate limiter")?;

// Visualise the chain
println!("{}", cot.visualise(&chain));

// Extract insights
let insights = cot.extract_insights(&chain);
```

**Reasoning Steps:**
1. Understanding
2. Analysis
3. Exploration
4. Design
5. Planning
6. Validation

### 3. Vibe Coding

Generate code from natural language:

```rust
use fusion_agentic_core::VibeEngine;

let vibe = VibeEngine::new();

// Detect patterns
let patterns = vibe.detect_patterns("parallel map transformation")?;

// Generate code
let code = vibe.generate_from_patterns(&patterns)?;

// Add custom patterns
vibe.add_pattern(my_custom_pattern);
```

**Pattern Categories:**
- Functional (map, filter, reduce)
- Object-Oriented (classes, inheritance)
- Algorithmic (sort, search)
- Data Structures (builders, trees)
- Concurrency (parallel, async)
- Error Handling (Result chains)
- Testing (unit tests)
- Performance (optimisations)

### 4. Code Excellence

Enforce quality standards:

```rust
use fusion_agentic_core::ExcellenceEnforcer;

let mut enforcer = ExcellenceEnforcer::new();

// Analyse code
let metrics = enforcer.analyse(code)?;

// Check specific metrics
println!("Readability: {}", metrics.readability);
println!("Security: {}", metrics.security);
println!("Overall: {}", metrics.overall_score);

// Validate against standards
enforcer.validate(code)?; // Returns error if below threshold

// Configure
enforcer.set_min_score(85.0);
enforcer.set_strict_mode(true);
```

**Quality Dimensions:**
- Readability (20% weight)
- Maintainability (25% weight)
- Performance (15% weight)
- Security (25% weight)
- Test Coverage (10% weight)
- Documentation (5% weight)

## Best Practices

### 1. Problem Decomposition

Always start with clear problem statements:

```rust
// Good
let solution = core.process_problem(
    "Implement a caching system with LRU eviction and TTL support"
)?;

// Less effective
let solution = core.process_problem("Make caching")?;
```

### 2. Iterative Refinement

Use the iteration feature for complex problems:

```rust
let initial = core.process_problem(problem)?;
let refined = core.iterate_solution(&initial, "Add error handling")?;
let finalised = core.iterate_solution(&refined, "Optimise performance")?;
```

### 3. Pattern Reuse

Build a library of custom patterns:

```rust
let mut vibe = VibeEngine::new();

// Add domain-specific patterns
vibe.add_pattern(auth_pattern);
vibe.add_pattern(database_pattern);
vibe.add_pattern(api_pattern);
```

### 4. Quality Gates

Integrate excellence checks into your workflow:

```rust
let code = generate_code()?;
let metrics = core.check_excellence(&code)?;

if !metrics.is_good() {
    eprintln!("Quality issues: {:#?}", metrics.recommendations);
    return Err(QualityError);
}
```

### 5. Confidence Monitoring

Track reasoning confidence:

```rust
let mut context = AgenticContext::new(problem);

// Monitor understanding
if context.understanding < 0.5 {
    // Need more information
}

// Monitor confidence
if context.confidence < 0.7 {
    // Consider alternative approaches
}
```

## Troubleshooting

### Low Confidence Scores

**Problem**: Solutions have low confidence scores.

**Solutions**:
- Provide more specific problem descriptions
- Break complex problems into smaller parts
- Use iterative refinement
- Check that your problem is well-defined

### Pattern Not Found

**Problem**: `PatternNotFound` error in vibe coding.

**Solutions**:
- Try different wording in your intent
- Check available patterns: `vibe.patterns_by_category(category)`
- Add a custom pattern for your use case
- Lower the matching threshold (advanced)

### Quality Validation Failures

**Problem**: Code fails excellence validation.

**Solutions**:
- Review the recommendations: `metrics.recommendations`
- Focus on security and maintainability (highest weights)
- Add documentation and tests
- Refactor complex functions
- Temporarily lower threshold for prototypes

### Out of Memory

**Problem**: Large reasoning chains cause memory issues.

**Solutions**:
- Limit chain depth
- Use streaming/incremental processing
- Clear reasoning history periodically
- Process sub-problems separately

### Slow Performance

**Problem**: Operations are slow.

**Solutions**:
- Pre-compile patterns
- Reuse `AgenticCore` instances
- Enable parallel processing features
- Reduce iteration limits for quick prototypes

## Support

For issues, questions, or contributions:
- GitHub: [Fusion Programming Language](https://github.com/QuantumSecureTechnologiesInc/Fusion-Programming-Language)
- Documentation: [docs/](../docs/)
- Examples: [examples/](../examples/)

## Next Steps

1. Explore the [Examples](../examples/basic_usage.rs)
2. Read the [Developer Guide](DeveloperGuide.md)
3. Check the [API Documentation](https://docs.rs/fusion-agentic-core)
4. Join the community discussions

---

**Version**: 0.1.0  
**Last Updated**: 2025-12-12  
**Licence**: MIT OR Apache-2.0
