// Fusion Core - Unified Type System and Compiler Infrastructure
//
// This crate combines:
// 1. Tri-brid Type System (Classical, Tensor, Quantum)
// 2. Compiler Infrastructure (Lexer, Parser, AST, VM)
//
// The fusion of these allows the compiler to understand and manipulate
// all three computational paradigms within a single framework.

pub mod compiler;
pub mod ops;
pub mod traits;
pub mod types;
pub mod vm;

// Re-export commonly used types
pub use types::classical::ClassicalType;
pub use types::hybrid::{FusionType, TensorDType, TensorType};
pub use types::quantum::{QuantumCircuit, QuantumGate, QuantumState, Qubit};
pub use types::tensor::{Matrix, Tensor};

// Re-export compiler infrastructure
pub use compiler::{Compiler, CompilerError, Lexer, Parser};
pub use vm::VM;

pub mod error;
pub use error::{FusionError, FusionResult};
