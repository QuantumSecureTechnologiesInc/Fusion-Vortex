use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Lexer error: {0}")]
    LexerError(String),

    #[error("Parser error: {0}")]
    ParserError(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
