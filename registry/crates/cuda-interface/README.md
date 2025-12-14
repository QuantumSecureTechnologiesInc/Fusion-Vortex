# CUDA Interface

High-level interface for interacting with CUDA devices and kernels from Fusion.

## Features
- Device management
- Memory allocation and transfer
- Kernel launch abstractions

## Requirements
- CUDA Toolkit installed
- Nvidia GPU

## Usage
```rust
use cuda_interface::Device;

let device = Device::get(0)?;
```
