// src/lib.rs - Library exports for testing and external use

pub mod ast;
pub mod async_runtime; // Async/Await Runtime
pub mod borrow_checker;
pub mod codegen;
pub mod crypto; // Hybrid cryptography module
pub mod docs; // Documentation generator
pub mod lexer;
pub mod lib_integration; // Library integration traits
pub mod lsp; // Language Server Protocol
pub mod ml; // Machine Learning & GPU
pub mod module_resolver; // Module resolution and dependency graph
pub mod network; // Secure Networking
pub mod optimization; // Optimization passes
pub mod package_manager; // Package management
pub mod parser;
pub mod quantum; // Quantum computing support
pub mod registry; // Package registry
pub mod security; // Security features (FIPS, fuzzing, verification, etc.)
pub mod semantic_analyzer;
pub mod stdlib;
pub mod wasm; // WebAssembly backend
pub mod web; // Web Framework
