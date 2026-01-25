# IBM Quantum Backend

Backend integration for running Fusion quantum circuits on IBM Quantum hardware via Qiskit Runtime or directly.

## Features

- IBM Quantum Experience integration
- Qiskit Runtime primitives
- Circuit execution on IBMQ devices

## Usage

```rust
use q_ibm_backend::IBMBackend;

let list = IBMBackend::list_devices(api_token)?;
```text