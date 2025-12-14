# Fusion LLM Auto Prompt

**Version:** 0.2.0  
**Type:** AI Utility  
**License:** MIT

## Overview

Fusion LLM Auto Prompt (`llm-auto-prompt`) is a specialized library for automatically generating and optimizing system prompts and few-shot examples for Large Language Models. It uses gradient-based or evolution-based search to find optimal prompts for a given task.

## Features

- **Prompt Optimization**: Automated search for high-performing prompts
- **Few-Shot Selection**: Dynamic selection of best examples
- **Template Generation**: Creates structured templates for complex tasks

## Usage

```rust
use llm_auto_prompt::{Optimizer, Task};

let task = Task::new("summarization", dataset);
let optimizer = Optimizer::new(task);
let best_prompt = optimizer.run().await?;
```

## Dependencies

- `fusion_ai_core`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
