# AWS Braket Backend

Backend integration for running Fusion quantum circuits on AWS Braket.

## Features

- Circuit submission to Amazon Braket
- Results retrieval
- Device selection (SV1, TN1, Rigetti, IonQ)

## Usage

```rust
use q_aws_backend::BraketBackend;

let backend = BraketBackend::new(region, bucket);
backend.run(circuit)?;
```text