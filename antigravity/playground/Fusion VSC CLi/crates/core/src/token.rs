#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    If,
    Else,
    Return,
    Extern,
    Struct,
    While,
    For,
    True,
    False,

    // Identifiers and Literals
    Identifier(String),
    Integer(i64),
    StringLiteral(String),

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Assign,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,

    // Delimiters
    LPren,
    RPren,
    LBrace,
    RBrace,
    SemiColon,
    Comma,
    Colon,
    Dot,

    // End of File
    EOF,

    // Error
    Illegal(char),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}
