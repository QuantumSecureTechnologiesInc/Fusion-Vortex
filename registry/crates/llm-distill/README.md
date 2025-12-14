# Fusion LLM Distill

**Version:** 0.2.0  
**Type:** CLI Tool  
**License:** MIT

## Overview

Fusion LLM Distill (`llm-distill`) is a command-line utility for running knowledge distillation jobs. It orchestrates the Teacher-Student training loop, managing data feeds and loss computation.

## Features

- **Teacher Management**: Connects to high-performance teacher models (e.g., GPT-4, Llama-3-70B)
- **Student Training**: Supervised fine-tuning of smaller models
- **Metrics**: Real-time tracking of KL divergence and loss

## Usage

```bash
fusion-distill --teacher "gpt-4" --student "llama-3-8b" --dataset "reasoning.jsonl"
```

## Dependencies

- `fusion_llm_distillation`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
