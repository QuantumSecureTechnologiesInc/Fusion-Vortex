# ML Demo Examples

**Location**: `examples/ml_demo/`  
**Status**: ✅ Working demonstrations  
**Purpose**: Showcase Fusion's ML capabilities

---

## Overview

These examples demonstrate Fusion's machine learning library with practical, runnable code showing tensor operations, neural networks, and training algorithms.

---

## Examples

### 1. Neural Network (neural_network.fu)

**Description**: Simple feedforward neural network for XOR problem

**Features**:
- Dense layer implementation
- Forward pass computation
- ReLU and Sigmoid activations
- Multi-layer network

**Usage**:
```bash
fusion_lang -i examples/ml_demo/neural_network.fu
```

**Output**:
- Network architecture summary
- Forward pass results
- Activation outputs

**Learning Goals**:
- Understanding neural network layers
- Forward propagation
- Activation functions
- Network composition

---

### 2. Linear Regression (linear_regression.fu)

**Description**: Linear regression with gradient descent

**Features**:
- Simple linear model (y = mx + b)
- Gradient descent optimization
- Training loop
- Loss computation (MSE)
- Prediction

**Usage**:
```bash
fusion_lang -i examples/ml_demo/linear_regression.fu
```

**Output**:
- Training progress (loss per epoch)
- Learned parameters (weight, bias)
- Test predictions

**Learning Goals**:
- Gradient descent
- Loss functions
- Parameter updates
- Model training

**Example Output**:
```
========================================
Linear Regression Demo
========================================
Learning the function: y = 2x + 1

Training data:
  (1.0, 3.0), (2.0, 5.0), (3.0, 7.0)
  (4.0, 9.0), (5.0, 11.0)

Training for 500 epochs...

Epoch 0, Loss: 45.2
Epoch 100, Loss: 2.1
Epoch 200, Loss: 0.3
Epoch 300, Loss: 0.05
Epoch 400, Loss: 0.01

Training complete!

Learned parameters:
  Weight (slope): 1.98
  Bias (intercept): 1.02

Expected: weight ≈ 2.0, bias ≈ 1.0

Testing predictions:
  Input: 6.0
  Predicted: 12.9
  Expected: 13.0 (2*6 + 1)

========================================
✅ Linear regression demo complete!
========================================
```

---

## ML Library Features Used

### Tensor Operations
- `Tensor::new()` - Create tensor
- `Tensor::zeros()` - Zero-initialized tensor
- `Tensor::get_at()` - Element access
- `Tensor::set_at()` - Element modification

### Activation Functions
- `relu()` - Rectified Linear Unit
- `sigmoid()` - Sigmoid activation
- `tanh_approx()` - Tanh approximation

### Operations
- `add_tensors()` - Element-wise addition
- `multiply_tensors()` - Element-wise multiplication
- `scalar_multiply()` - Scalar multiplication

---

## Code Structure

### Neural Network Example

```fusion
class DenseLayer {
    input_size: int;
    output_size: int;
    weights: Tensor;
    bias: Tensor;
    learning_rate: float;
}

impl DenseLayer {
    fn new(input_size: int, output_size: int, lr: float) -> DenseLayer;
    fn forward(self, input: Tensor) -> Tensor;
}
```

### Linear Regression Example

```fusion
class LinearRegression {
    weight: float;
    bias: float;
    learning_rate: float;
}

impl LinearRegression {
    fn new(lr: float) -> LinearRegression;
    fn predict(self, x: float) -> float;
    fn train_step(mut self, x: float, y_true: float);
    fn train(mut self, x_data: Vector<float>, y_data: Vector<float>, epochs: int);
}
```

---

## Running the Examples

### Prerequisites
- Fusion compiler installed
- Standard library available
- ML library (stdlib/ml/)

### Compilation

**Native (LLVM)**:
```bash
fusion_lang -i examples/ml_demo/neural_network.fu
fusion_lang -i examples/ml_demo/linear_regression.fu
```

**WebAssembly** (future):
```bash
fusion_lang -i examples/ml_demo/neural_network.fu --target wasm -o nn.wasm
```

---

## Future Enhancements

### Planned Examples
- [ ] Logistic regression (binary classification)
- [ ] K-means clustering
- [ ] Convolutional neural network (CNN)
- [ ] Recurrent neural network (RNN)
- [ ] Autoencoder
- [ ] Transfer learning demo

### Planned Features
- [ ] GPU acceleration (@gpu_accelerated)
- [ ] Automatic differentiation
- [ ] Training visualization
- [ ] Model saving/loading
- [ ] Pre-trained models

---

## Performance Notes

**Current Implementation**:
- CPU-only execution
- Vector-based tensor storage
- Simple gradient decent
- Forward pass only (no backprop yet)

**Future Optimizations**:
- GPU kernel generation
- SIMD vectorization
- Batch processing
- Optimized matrix multiplication

---

## Learning Resources

**Concepts Demonstrated**:
- Neural network architecture
- Forward propagation
- Gradient descent
- Loss functions
- Activation functions
- Parameter optimization

**Next Steps**:
1. Run the examples
2. Modify parameters (learning rate, epochs)
3. Try different architectures
4. Implement backpropagation (Phase 4.2)
5. Add GPU acceleration (Phase 4.3)

---

## Contributing

Want to add more examples?
1. Create new .fu file in examples/ml_demo/
2. Follow the existing structure
3. Add documentation
4. Submit pull request

---

**Status**: ✅ **2 Working Examples**  
**Difficulty**: Beginner to Intermediate  
**Estimated Time**: 15-30 minutes  
**Prerequisites**: Basic ML knowledge helpful

**These examples showcase Fusion's potential for machine learning applications!** 🚀
