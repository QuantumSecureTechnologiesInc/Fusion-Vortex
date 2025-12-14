# Fusion LLM Logits Processor

**Version:** 0.2.0  
**Type:** Generation Pipeline  
**License:** MIT

## Overview

Fusion LLM Logits Processor (`fusion_llm_logits_processor`) is a flexible pipeline for modifying model logits before sampling. It implements penalties, biases, and constraints to control generation output.

## Features

- **Repetition Penalty**: Penalize already generated tokens
- **Frequency/Presence Penalty**: OpenAI-style penalties
- **Logit Bias**: Force or ban specific tokens
- **Temperature**: Scaling logits for entropy control

## Usage

```rust
use fusion_llm_logits_processor::{LogitsProcessor, RepetitionPenalty};

let mut processor = LogitsProcessor::new();
processor.push(RepetitionPenalty::new(1.1));

let processed_logits = processor.apply(logits, &input_ids);
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
