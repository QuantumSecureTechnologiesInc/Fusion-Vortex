# Fusion v2.0 Vortex User Guide

## Introduction

Fusion v2.0 Vortex is a **self-hosting, quantum-native, AI-integrated** systems programming language. This guide covers core language features and the unique capabilities of the Vortex edition.

## Language Basics

### Functions

```fusion
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() {
    let result = add(10, 20);
    print(result);  // Output: 30
}
```

### Variables

```fusion
let x: int = 10;        // Immutable
let mut y: int = 20;    // Mutable
y = y + x;              // OK: y is mutable
```

### Control Flow

```fusion
// If-else
if x > 5 {
    return x;
} else {
    return 0;
}

// While loop
let mut i = 0;
while i < 10 {
    print(i);
    i = i + 1;
}

// For loop
for item in collection {
    process(item);
}
```

### Classes and Traits

```fusion
class Point {
    x: float;
    y: float;
}

impl Point {
    fn new(x: float, y: float) -> Point {
        return Point { x: x, y: y };
    }

    fn distance(self, other: Point) -> float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return sqrt(dx * dx + dy * dy);
    }
}

trait Drawable {
    fn draw(self);
}

impl Drawable for Point {
    fn draw(self) {
        draw_circle(self.x, self.y, 5.0);
    }
}
```

---

## Vortex-Specific Features

### Intent-Driven Execution

Annotate functions with execution intent for optimal hardware scheduling:

```fusion
use compiler::intent::Intent;

#[intent(Critical)]  // HFT: Always CPU, <10μs latency
fn process_order(order: Order) -> Trade { ... }

#[intent(HighThroughput)]  // AI: Prefers GPU
fn train_model(data: Tensor) -> Model { ... }

#[intent(Precision)]  // Science: Extended precision
fn simulate_quantum(circuit: Circuit) -> StateVector { ... }

#[intent(Background)]  // Low priority
fn log_metrics(data: Metrics) { ... }
```

### Post-Quantum Cryptography

Built-in quantum-resistant cryptographic operations:

```fusion
use compiler::pqc::{KyberKeypair, DilithiumSign, HybridKeypair};

// Generate hybrid keypair (Classical + Post-Quantum)
let keypair = HybridKeypair::generate();

// Kyber768 Key Encapsulation
let kyber = KyberKeypair::generate_768();
let kem = KyberKEM::new(KyberSecurityLevel::Kyber768);
let encap = kem.encapsulate(peer_public_key);

// Dilithium3 Digital Signature
let dsa = DilithiumSign::new(DilithiumSecurityLevel::Dilithium3);
let signature = dsa.sign(secret_key, message);
let valid = dsa.verify(public_key, message, signature);
```

### Quantum Computing

Native quantum circuit programming:

```fusion
use std::quantum;

let mut circuit = quantum::QuantumCircuit::new(2);
circuit.h(0);       // Hadamard gate on qubit 0
circuit.cx(0, 1);   // CNOT (control: 0, target: 1)

let result = circuit.execute().await?;
print(result.state_vector());
```

### AI/ML Tensors

First-class tensor operations with automatic GPU acceleration:

```fusion
use std::ai;

let tensor = ai::Tensor::randn([1000, 1000]);
let result = tensor.matmul(tensor.transpose()).relu();
print("Executed on:", result.device());  // CUDA:0 or CPU
```

---

## Self-Hosting Compiler

Fusion v2.0 Vortex includes a complete compiler written in pure Fusion:

```fusion
use compiler::{compile, lex, parse};

// Compile Fusion source code
let source = "fn main() { print(42); }";
let result = compile(source, "main.fu");

if result.success {
    let bytecode = result.bytecode.unwrap();
    vm::execute(bytecode);
}
```

Compiler modules in `src/compiler/`:

| Module       | Purpose                   |
| ------------ | ------------------------- |
| `lexer.fu`   | Hand-written tokenizer    |
| `parser.fu`  | Recursive descent parser  |
| `sema.fu`    | Type checking & inference |
| `codegen.fu` | Bytecode + x86_64 output  |
| `intent.fu`  | Intent-driven scheduling  |
| `pqc.fu`     | Kyber/Dilithium crypto    |

---

---

## Enterprise Standard Library

Fusion v2.0 Vortex includes a robust set of native modules for building production systems:

### Structured Logging (`std::log`)

```fusion
use std::log;
let logger = log::Logger::new(log::Level::Info).with_file("app.log");
logger.info("Service started on port 8080");
```

### JSON Configuration (`std::json`)

```fusion
use std::json;
let config = json::ConfigParser::new(fs::read_to_string("config.json"));
let port = config.get_key("port");
```

### HTTP Server (`std::http`)

```fusion
use std::http;
let server = http::SimpleServer::new(8080);
server.serve();
```

---

## Native Standard Library

The `stdlib/` provides pure Fusion implementations for system interaction:

### File System (`std::fs`)

```fusion
use std::fs;
let content = fs::read_to_string("config.toml");
fs::write_string("log.txt", "Entry 1");
```

### Networking (`std::net`)

```fusion
use std::net;
let mut stream = net::TcpStream::connect("example.com:80");
stream.write("GET / HTTP/1.1\r\n\r\n");
```

### Process (`std::process`)

```fusion
use std::process;
let path = process::env("PATH");
process::exit(0);
```

---

## Next Steps

- **[Quick Start Guide](QuickStartGuide.md)** - Get running in 10 minutes
- **[Developer Guide](DeveloperGuide.md)** - Architecture deep-dive
- **[Feature Index](../features/FEATURES_INDEX.md)** - Explore all capabilities
