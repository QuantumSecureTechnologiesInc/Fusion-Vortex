// Compiler infrastructure modules from old fusion-compiler crate
pub mod ast;
pub mod chunk;
pub mod compiler;
pub mod error;
pub mod function;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod token;
pub mod type_checker;
pub mod typechecker;
pub mod value;

// Re-exports for backwards compatibility
pub use self::compiler::Compiler;
pub use self::error::CompilerError;
pub use self::lexer::Lexer;
pub use self::parser::Parser;
