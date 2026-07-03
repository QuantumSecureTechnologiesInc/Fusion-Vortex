// src/wasm/mod.rs - WebAssembly Backend Module
// NOTE: Requires wasm-encoder feature. Gated to avoid conflicts.

#[cfg(feature = "wasm")]
pub mod codegen;
#[cfg(feature = "wasm")]
pub mod types;

#[cfg(feature = "wasm")]
pub use codegen::WasmCodeGenerator;