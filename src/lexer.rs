// src/lexer.rs - Logos Lexer Adapter

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+")] // Skip whitespace
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("class")]
    Class,
    #[token("extern")]
    Extern,
    #[token("pub")]
    Pub,
    #[token("mod")]
    Mod,
    #[token("use")]
    Use,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Identifiers and Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    #[regex(r#""([^"\\]|\\["\\bnfrt])*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLit(String),

    // Symbols
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("->")]
    Arrow,
    #[token("=")]
    Assign,
    #[token(".")]
    Dot,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,

    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,

    // Comments
    #[regex(r"//.*", logos::skip)]
    Comment,
}
