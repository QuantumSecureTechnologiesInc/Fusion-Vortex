# Fusion Python Interop

**Version:** 0.2.0  
**Type:** Language Bridge  
**License:** MIT

## Overview

Fusion Python Interop (`interop-python`) provides robust bindings to the Python interpreter (CPython). It allows Fusion to import modules, call functions, and share memory (Zero-Copy with NumPy) with Python.

## Features

- **PyO3 Integration**: wrapper around PyO3 for Fusion safety
- **GIL Management**: Safe abstractions for Global Interpreter Lock
- **NumPy Support**: Zero-copy tensor sharing
- **VirtualEnv**: Support for activating specific virtual environments

## Usage

```rust
use interop_python::{Python, PyModule};

let py = Python::acquire_gil();
let numpy = PyModule::import(py, "numpy")?;
let result = numpy.call1("sqrt", (2.0,))?;

println!("Sqrt(2) = {}", result);
```

## Dependencies

- `pyo3`
- `fusion_core`

## Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md)
