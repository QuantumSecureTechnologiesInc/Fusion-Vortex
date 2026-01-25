# Fusion LLM Prompt Tuning

**Version:** 0.2.0
**Type:** Fine-Tuning
**License:** MIT

## Overview

Fusion LLM Prompt Tuning (`llm-prompt-tuning`) implements Parameter-Efficient Fine-Tuning (PEFT) via prompt tuning. It learns "soft prompts" (trainable embedding vectors) prepended to the input, allowing task adaptation without modifying model weights.

## Features

- **Soft Prompts**: Trainable continuous vectors
- **Task Switching**: Switch tasks by swapping soft prompt embeddings
- **Integrations**: Compatible with `fusion_llm_inference_engine`

## Usage

```rust
use llm_prompt_tuning::{Tuner, Config};

let tuner = Tuner::new(model, Config::default());
let soft_prompt = tuner.train(dataset).await?;
soft_prompt.save("my_task.bin")?;
```text

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)