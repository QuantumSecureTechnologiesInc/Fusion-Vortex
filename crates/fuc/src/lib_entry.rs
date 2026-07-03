//! Fusion compiler library.
/// AST definitions.
mod ast;
/// Borrow Checker (Vortex).
mod borrowck;
/// CLI helpers.
mod cli;
/// Code generation backends.
mod codegen;
/// IR lowering.
mod ir;
/// Lexer.
mod lexer;
/// Language Server Protocol.
mod lsp;
/// Optimizer passes.
mod optimizer;
/// Parser.
mod parser;
/// Semantic analysis.
mod sema;
/// WASM code generation backend.
mod wasm;

// --- PREVIOUS ARCHITECTURAL COMPONENTS ---
/// Build System & Package Manager (Runtimes)
mod forge;
/// Code Formatter
mod fmt;
/// Macro Expansion
mod macros;
/// Static Single Assignment (SSA) IR
mod ssa;
/// Borrow Checker
mod borrowck;
/// Language Server Protocol
mod lsp;

// --- NEW ARCHITECTURAL COMPONENTS ---
/// Fusion Async Reactor (HyperRing)
mod reactor;
/// Fusion Garbage Collector (Polyglot)
mod gc;
/// Test Framework & Fuzzing
mod test;
/// Compiler Profiler
mod profiler;

// --- EXTREME ARCHITECTURE / ADVANCED PASSES ---
/// Function Inliner (IR Optimization)
mod inliner;
/// Generic Monomorphization Pass
mod monomorphize;
/// Runtime Type Information (Reflection)
mod rtti;
/// Advanced Diagnostics & Suggestions
mod diagnostics;

/// Re-export backend trait.
pub use crate::codegen::Backend;