# Fusion AI Models

**Version:** Workspace  
**Type:** AI Model Runtime  
**License:** MIT / Apache 2.0 Dual License

## Overview

Fusion AI Models (`fusion-ai-models`) provides the runtime implementation for executing local AI models. It supports various backends including specific optimized kernels for inference.

## Backends

- **llama.cpp**: Efficient CPU/GPU inference for LLaMA-based models
- **ONNX Runtime**: General purpose inference for standard models
- **TorchScript**: PyTorch model execution (optional)

## Features

- **Quantization Support**: Efficient execution of 4-bit/8-bit quantized models
- **Hardware Acceleration**: Auto-detection of CUDA, Metal, and ROCm
- **Model Management**: Loading, unloading, and memory tracking

## Usage

```rust
use fusion_ai_models::{ModelLoader, Backend};

async fn load_model() -> Result<(), anyhow::Error> {
    let loader = ModelLoader::new();
    let model = loader.load("llama-2-7b.gguf", Backend::LlamaCpp).await?;
    
    // Ready for inference
    Ok(())
}
```

## Configuration

Supports feature flags to enable specific backends:

```toml
[dependencies]
fusion-ai-models = { workspace = true, features = ["llama-cpp", "onnx"] }
```

## Dependencies

- `fusion_runtime_core`
- `serde`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
