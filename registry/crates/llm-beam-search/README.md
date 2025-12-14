# Fusion LLM Beam Search

**Version:** 0.2.0  
**Type:** Decoding Algorithm  
**License:** MIT

## Overview

Fusion LLM Beam Search (`llm-beam-search`) implements efficient Beam Search decoding for LLM inference. It allows for exploring multiple potential generation paths simultaneously to find the highest probability sequence.

## Features

- **Diverse Beam Search**: Includes diversity penalties to prevent repetitive loops
- **Constrained Decoding**: Supports enforcing grammars or constraints
- **Parallel Expansion**: Expands beam candidates in parallel on GPU

## Usage

```rust
use llm_beam_search::{BeamSearch, Config};

let config = Config { beam_width: 5, ..Default::default() };
let search = BeamSearch::new(model, config);
let output = search.generate(input_tokens).await?;
```

## Dependencies

- `fusion_ai_core`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
