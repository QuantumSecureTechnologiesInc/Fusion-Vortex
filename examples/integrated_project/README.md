# Integrated Project - ML + Package Manager Demo

**Project**: MNIST Digit Classifier
**Purpose**: Demonstrate Fusion's package management + ML capabilities
**Status**: ✅ Complete integration example

---

## Overview

This project demonstrates:

- Using `fusion.toml` for dependency management
- Building ML models with Fusion's ML library
- Package-based project structure
- Professional development workflow

---

## Project Structure

```text
ml-mnist-classifier/
├── fusion.toml           # Package manifest
├── fusion.lock           # Dependency lock file
├── src/
│   ├── main.fu          # Entry point
│   ├── model.fu         # CNN model definition
│   ├── data.fu          # Data loading
│   └── train.fu         # Training loop
├── tests/
│   └── model_tests.fu   # Unit tests
└── README.md            # This file
```

---

## Dependencies

Defined in `fusion.toml`:

```toml
[dependencies]
collections = "^1.0"      # HashMap, HashSet, Vector
fusion-ml = "^0.1"        # ML library (Tensor, layers)
data-utils = "^0.1"       # Data loading utilities
```

---

## Building

### Using Package Manager

```

# Initialize project (if starting from scratch)

fusion new ml-mnist-classifier
cd ml-mnist-classifier

# Add dependencies

fusion add collections
fusion add fusion-ml
fusion add data-utils

# Install dependencies

fusion build

# This will

# 1. Read fusion.toml

# 2. Resolve dependencies

# 3. Download packages

# 4. Generate fusion.lock

# 5. Compile with dependencies

```

### Manual Build

```

# Direct compilation

fusion_lang -i src/main.fu -o mnist_classifier

# Run

./mnist_classifier
```

---

## Running

```

# Run with package manager

fusion run

# Or directly

./mnist_classifier
```

---

## Usage Example

```fusion
// src/main.fu
use collections::Vector;
use fusion_ml::tensor::Tensor;
use fusion_ml::layers::Conv2D;
use fusion_ml::optimizers::AdamOptimizer;
use data_utils::MnistLoader;

fn main() -> int {
    // Load MNIST data
    let mut loader = MnistLoader::new("data/mnist");
    let train_data = loader.load_training();

    // Create CNN model
    let mut model = create_mnist_cnn();

    // Create optimizer
    let mut optimizer = AdamOptimizer::new(0.001);

    // Train
    train_model(model, train_data, optimizer, 10);

    // Evaluate
    let accuracy = evaluate_model(model, loader.load_test());

    println("Final accuracy: " + accuracy + "%");

    return 0;
}
```

---

## Features Demonstrated

### Package Management

- ✅ Dependency declaration
- ✅ Semantic versioning (^1.0, ~0.5)
- ✅ Lock file generation
- ✅ Dependency resolution
- ✅ Cached package downloads

### ML Capabilities

- ✅ Tensor operations
- ✅ Convolutional layers (Conv2D)
- ✅ Pooling layers (MaxPool2D)
- ✅ Dropout regularization
- ✅ Dense layers
- ✅ Activation functions (ReLU, Sigmoid)
- ✅ Optimizers (SGD, Adam, RMSprop)
- ✅ Training loops
- ✅ Data loading

### Development Workflow

- ✅ Project initialization (`fusion new`)
- ✅ Dependency management (`fusion add`)
- ✅ Building (`fusion build`)
- ✅ Running (`fusion run`)
- ✅ Testing (`fusion test`)
- ✅ Distribution (`fusion publish`)

---

## Expected Output

```text
========================================
MNIST CNN Classifier
========================================

📦 Loading dependencies...
  ✓ collections@1.0.0
  ✓ fusion-ml@0.1.0
  ✓ data-utils@0.1.0

📊 Loading MNIST dataset...
  Training samples: 60,000
  Test samples: 10,000

🧠 Building CNN model...
  Conv1: 1->32 (3x3)
  Pool1: 2x2
  Conv2: 32->64 (3x3)
  Pool2: 2x2
  FC1: 3136->128
  FC2: 128->10

🚀 Training for 10 epochs...
  Epoch 1/10 - Loss: 0.523 - Acc: 84%
  Epoch 2/10 - Loss: 0.234 - Acc: 93%
  Epoch 3/10 - Loss: 0.156 - Acc: 95%
  ...
  Epoch 10/10 - Loss: 0.045 - Acc: 99%

✅ Training complete!

📈 Evaluating on test set...
  Test accuracy: 98.5%

========================================
```

---

## Lock File (fusion.lock)

Generated automatically on first build:

```toml
version = 1
generated_at = "2025-12-07T10:00:00Z"

[[package]]
name = "collections"
version = "1.0.0"
source = "registry+https://packages.fusion-lang.org"
checksum = "sha256:abc123..."
dependencies = []

[[package]]
name = "fusion-ml"
version = "0.1.0"
source = "registry+https://packages.fusion-lang.org"
checksum = "sha256:def456..."
dependencies = ["collections"]

[[package]]
name = "data-utils"
version = "0.1.0"
source = "registry+https://packages.fusion-lang.org"
checksum = "sha256:ghi789..."
dependencies = ["collections"]
```

---

## Benefits of This Approach

### For Developers

- **Easy dependency management** - No manual downloads
- **Reproducible builds** - Lock file ensures consistency
- **Version control** - Semantic versioning support
- **Fast iteration** - Cached dependencies

### For Teams

- **Consistent environments** - Everyone uses same versions
- **Easy onboarding** - Just `fusion build`
- **Dependency tracking** - Clear what's being used
- **Security** - Checksum verification

### For the Ecosystem

- **Code reuse** - Share ML models as packages
- **Standardization** - Common interfaces
- **Collaboration** - Easy to contribute
- **Distribution** - Simple package publishing

---

## Next Steps

### Enhance the Model

- Add data augmentation
- Implement early stopping
- Add learning rate scheduling
- Experiment with architectures

### Scale Up

- Train on full MNIST dataset
- Add validation dataset
- Implement checkpointing
- Add TensorBoard logging

### Deploy

- Export to WASM for browser
- Create REST API
- Build web interface
- Containerize with Docker

---

## Testing

```

# Run tests

fusion test

# Specific test

fusion test model_tests

# With coverage

fusion test --coverage
```

---

## Publishing

```

# Build release version

fusion build --release

# Publish to package registry

fusion publish

# This will

# 1. Build optimized binaries

# 2. Run tests

# 3. Generate documentation

# 4. Upload to registry

```

---

## Requirements

- Fusion compiler v0.1.0+
- Package manager enabled
- Internet connection (for first build)
- ~50MB disk space for dependencies

---

## License

MIT License - See LICENSE file

---

## Support

- Documentation: <https://fusion-lang.org/docs>
- Issues: <https://github.com/fusion-lang/fusion/issues>
- Community: <https://discord.gg/fusion-lang>

---

**This example showcases Fusion's modern development experience!** 🚀
