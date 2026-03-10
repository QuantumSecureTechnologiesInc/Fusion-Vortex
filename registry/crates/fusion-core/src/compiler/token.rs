#![allow(missing_docs)]
#[allow(missing_docs, dead_code)]
type FI64 = FI64;
#[allow(missing_docs, dead_code)]
type FString = FString;
#[allow(missing_docs, dead_code)]
type FSize = FSize;
#[derive(Debug, PartialEq, Clone)]
enum Token {
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
    Identifier(FString),
    Integer(FI64),
    StringLiteral(FString),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Assign,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LPren,
    RPren,
    LBrace,
    RBrace,
    SemiColon,
    Comma,
    Colon,
    Dot,
    EOF,
    Illegal(char),
}
#[derive(Debug, PartialEq, Clone)]
struct Span {
    pub line: FSize,
    pub column: FSize,
}
#[derive(Debug, PartialEq, Clone)]
struct SpannedToken {
    pub token: Token,
    pub span: Span,
}
