# Fusion Attention

**Version:** 0.2.0  
**Type:** Neural Network Layer  
**License:** MIT

## Overview

Fusion Attention (`fusion_attention`) implements high-performance Multi-Head Attention mechanisms, the core building block of Transformer models. It is optimized for Fusion's Tensor system and supports both inference and training operations.

## Features

- **Multi-Head Attention**: Scaled dot-product attention
- **Self-Attention**: Optimized self-attention implementation
- **Masking**: Causal masking for autoregressive models
- **KV Caching**: Efficient caching for incremental decoding

## Usage

```rust
use fusion_attention::MultiHeadAttention;
use fusion_ai_core::Tensor;

let embed_dim = 512;
let num_heads = 8;
let attention = MultiHeadAttention::new(embed_dim, num_heads)?;

// Forward pass
let input = Tensor::randn(&[32, 10, embed_dim]); // [Batch, Seq, Dim]
let output = attention.forward(&input)?;
```

## Mathematics

Implements the standard attention formula:
$$ \text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V $$

## Dependencies

- `fusion_ai_core`: Tensor operations
- `fusion_core`: Error handling

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
