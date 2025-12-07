// src/lib.rs - Library exports for testing and external use

pub mod ast;
pub mod borrow_checker;
pub mod codegen;
pub mod crypto; // Hybrid cryptography module
pub mod lexer;
pub mod lsp; // Language Server Protocol
pub mod module_resolver; // Module resolution and dependency graph
pub mod parser;
pub mod semantic_analyzer;
pub mod stdlib;
pub mod wasm; // WebAssembly backend
