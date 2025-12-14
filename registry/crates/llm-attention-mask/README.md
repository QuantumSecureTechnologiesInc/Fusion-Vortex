# Fusion LLM Attention Mask

**Version:** 0.2.0  
**Type:** AI Utility  
**License:** MIT

## Overview

Fusion LLM Attention Mask (`fusion_llm_attention_mask`) provides optimized utilities for generating attention masks used in Transformer models. It supports various masking strategies required for training and inference.

## Features

- **Causal Masking**: For autoregressive generation (GPT-style)
- **Sliding Window**: For local attention patterns (Longformer/Mistral)
- **Padding Masks**: Efficient handling of variable sequence lengths
- **Block Sparse**: Optimized block-sparse masks for long contexts

## Usage

```rust
use fusion_llm_attention_mask::{MaskGenerator, MaskType};

let seq_len = 1024;
let mask = MaskGenerator::generate(seq_len, MaskType::Causal);
// Returns a boolean or float tensor
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
