// src/lsp/mod.rs - Updated LSP module with all components

pub mod diagnostics;
pub mod enhanced;
pub mod inlay_hints;
pub mod navigation;
pub mod refactoring;
pub mod semantic_tokens;
pub mod server;

// Re-export main types
pub use enhanced::{CodeActionProvider, RenameOperation, SymbolIndex, SymbolInfo};
pub use server::FusionLanguageServer;
