# Fusion LLM Prompt Prefill

**Version:** 0.2.0  
**Type:** Optimization  
**License:** MIT

## Overview

Fusion LLM Prompt Prefill (`llm-prompt-prefill`) optimizes the processing of long system prompts and context. It enables "prefix caching" where common prompt segments are processed once and cached, reducing latency for subsequent requests.

## Features

- **Prefix Caching**: Reuse KV cache for shared prompt prefixes
- **Context Management**: Efficient handling of multi-turn conversations
- **Serialization**: Save and load processed prompt states

## Usage

```rust
use llm_prompt_prefill::{PrefillManager, Prompt};

let manager = PrefillManager::new();
let system_prompt = Prompt::from_text(LONG_SYSTEM_PROMPT);
let cache_id = manager.prefill(&system_prompt).await?;

// Use cache_id for subsequent requests
```

## Dependencies

- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
