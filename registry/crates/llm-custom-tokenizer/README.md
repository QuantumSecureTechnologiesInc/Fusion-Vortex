# Fusion LLM Custom Tokenizer

**Version:** 0.2.0  
**Type:** Tools  
**License:** MIT

## Overview

Fusion LLM Custom Tokenizer (`llm-custom-tokenizer`) allows researchers and developers to define, train, and use custom tokenizers for specialized domains (e.g., biology, code, foreign languages) within the Fusion ecosystem.

## Features

- **BPE & Unigram**: standard algorithms supported
- **Special Tokens**: Flexible handling of control tokens
- **Training**: Efficient production of vocabularies from raw text

## Usage

```rust
use llm_custom_tokenizer::Trainer;

let trainer = Trainer::new();
let tokenizer = trainer.train_from_files(&["corpus.txt"], 32000)?;
tokenizer.save("custom.model")?;
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
