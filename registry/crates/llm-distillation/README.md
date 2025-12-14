# Fusion LLM Distillation

**Version:** 0.2.0  
**Type:** ML Framework  
**License:** MIT

## Overview

Fusion LLM Distillation (`fusion_llm_distillation`) provides the core logic and algorithms for compressing Large Language Models via Knowledge Distillation. It optimizes the transfer of reasoning capabilities from large teacher models to smaller, faster student models.

## Features

- **Logit-based Distillation**: Minimizes KL divergence between teacher and student logits
- **Hidden State Alignment**: Matches internal representations (Hinton loss)
- **Attention Transfer**: Transfers attention maps to guide student focus
- **Zero-Shot Distillation**: Synthetic data generation on the fly

## Usage

```rust
use fusion_llm_distillation::{Distiller, Config};

let config = Config::default();
let distiller = Distiller::new(teacher_model, student_model, config);

while let Some(batch) = dataset.next().await {
    let loss = distiller.step(batch).await?;
    println!("Distillation Loss: {}", loss);
}
```

## Dependencies

- `fusion_core`
- `fusion_ai_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
