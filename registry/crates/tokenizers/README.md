# Fusion Tokenizers

**Version:** 0.2.0
**Type:** Text Processing
**License:** MIT

## Overview

Fusion Tokenizers (`fusion_tokenizers`) provides efficient text tokenization for LLMs and NLP tasks. It supports Byte-Pair Encoding (BPE), WordPiece, and SentencePiece algorithms.

## Features

- **Performance**: High-speed, parallel tokenization
- **Compatibility**: Supports loading HuggingFace `tokenizer.json`
- **Reversibility**: Lossless decoding of tokens back to text

## Usage

```rust
use fusion_tokenizers::Tokenizer;

let enc = Tokenizer::from_file("tokenizer.json")?;
let tokens = enc.encode("Hello Fusion", false)?;

println!("IDs: {:?}", tokens.get_ids());
```text

## Dependencies

- `fusion_std`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)