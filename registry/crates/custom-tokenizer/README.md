# Custom Tokenizer

A flexible tokenizer library for Fusion, designed to handle custom grammars and domain-specific languages.

## Features
- Customizable token definitions
- Regex integration
- High-performance lexing

## Usage
```rust
use custom_tokenizer::Tokenizer;

let tokenizer = Tokenizer::new(rules);
let tokens = tokenizer.tokenize("source code");
```
