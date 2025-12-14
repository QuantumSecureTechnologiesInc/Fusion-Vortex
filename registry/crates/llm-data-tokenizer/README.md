# Fusion LLM Data Tokenizer

**Version:** 0.2.0  
**Type:** Data Pipeline  
**License:** MIT

## Overview

Fusion LLM Data Tokenizer (`fusion_llm_data_tokenizer`) is a high-throughput, parallel tokenization engine designed for pre-processing massive datasets for LLM training.

## Features

- **Parallel Processing**: multi-threaded tokenization for multiple files
- **Memory Efficient**: Streaming processing prevents RAM exhaustion
- **Sharding**: Automatically shards output for distributed training
- **Format Support**: Handles JSONL, Parquet, and raw text

## Usage

```rust
use fusion_llm_data_tokenizer::{BatchTokenizer, Config};

let config = Config::default();
let tokenizer = BatchTokenizer::new(config);
tokenizer.process_dir("raw_data/", "tokenized_data/").await?;
```

## Dependencies

- `fusion_core`
- `fusion_llm_tokenizers`
- `fusion_runtime_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
