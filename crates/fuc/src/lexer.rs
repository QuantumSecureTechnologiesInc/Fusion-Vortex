//! Fusion Lexer - Token definitions and tokenization.
//! The full lexer implementation lives in lexer.fu (self-hosted Fusion source).
//! This Rust stub provides the type definitions and a minimal tokenizer.

use crate::types::*;

/// Token definitions for the Fusion language.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    KwFn,
    KwLet,
    KwReturn,
    KwIf,
    KwElse,
    KwWhile,
    KwFor,
    KwIn,
    KwMatch,
    KwImpl,
    KwTrait,
    KwWhere,
    KwConst,
    KwStatic,
    KwUse,
    KwMod,
    KwPub,
    KwAsync,
    KwAwait,
    KwStruct,
    KwEnum,
    KwType,
    KwExtern,
    KwImport,
    Identifier(FString),
    IntLiteral(FI64),
    StringLiteral(FString),
    Colon,
    ColonColon,
    Semicolon,
    Assign,
    Equals,
    NotEquals,
    Bang,
    Or,
    Pipe,
    And,
    FatArrow,
    Arrow,
    Range,
    Less,
    Greater,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    LessEqual,
    GreaterEqual,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Ellipsis,
    Dot,
    Question,
    Hash,
    TypeInt,
    TypeBool,
    TypeString,
    TypeVoid,
    True,
    False,
    Error,
}

/// Spanned token: (Token, Span) where Span is byte range.
pub type SpannedToken = (Token, std::ops::Range<FSize>);

/// Token stream produced by the lexer.
pub struct TokenStream {
    pub tokens: FVec<SpannedToken>,
}

/// Lexes the given source string into a stream of tokens with spans.
pub fn lex(source: &str) -> TokenStream {
    let tokens = tokenize_inner(source);
    TokenStream { tokens }
}

/// Alias for backward compatibility with aspirational code.
pub fn tokenize(source: &str) -> TokenStream {
    lex(source)
}

/// Minimal tokenizer that handles basic tokens for bootstrap purposes.
fn tokenize_inner(source: &str) -> FVec<SpannedToken> {
    let bytes = source.as_bytes();
    let mut tokens: FVec<SpannedToken> = Vec::new();
    let mut i = 0usize;
    let len = bytes.len();

    // Skip shebang
    if len >= 2 && bytes[0] == b'#' && bytes[1] == b'!' {
        while i < len && bytes[i] != b'\n' {
            i += 1;
        }
    }

    while i < len {
        let ch = bytes[i];

        // Skip whitespace
        if ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n' || ch == 0x0c {
            i += 1;
            continue;
        }

        // Skip line comments
        if ch == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            i += 2;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }

        // Skip block comments
        if ch == b'/' && i + 1 < len && bytes[i + 1] == b'*' {
            i += 2;
            while i + 1 < len {
                if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }

        let start = i;

        // Identifiers and keywords
        if is_ident_start(ch) {
            i += 1;
            while i < len && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let text = &source[start..i];
            tokens.push((keyword_or_ident(text), start..i));
            continue;
        }

        // Number literals
        if ch >= b'0' && ch <= b'9' {
            i += 1;
            while i < len && bytes[i] >= b'0' && bytes[i] <= b'9' {
                i += 1;
            }
            let text = &source[start..i];
            if let Ok(parsed) = text.parse::<i64>() {
                tokens.push((Token::IntLiteral(parsed), start..i));
            } else {
                tokens.push((Token::Error, start..i));
            }
            continue;
        }

        // String literals
        if ch == b'"' {
            i += 1;
            while i < len {
                if bytes[i] == b'\\' {
                    i += 2;
                    continue;
                }
                if bytes[i] == b'"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            if i >= start + 2 {
                let literal = &source[start + 1..i - 1];
                tokens.push((Token::StringLiteral(literal.to_string()), start..i));
            } else {
                tokens.push((Token::Error, start..i));
            }
            continue;
        }

        // Multi-char tokens
        if i + 2 < len && bytes[i] == b'.' && bytes[i + 1] == b'.' && bytes[i + 2] == b'.' {
            tokens.push((Token::Ellipsis, i..i + 3));
            i += 3;
            continue;
        }
        if i + 1 < len && bytes[i] == b'.' && bytes[i + 1] == b'.' {
            tokens.push((Token::Range, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b':' && bytes[i + 1] == b':' {
            tokens.push((Token::ColonColon, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'-' && bytes[i + 1] == b'>' {
            tokens.push((Token::Arrow, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'=' && bytes[i + 1] == b'=' {
            tokens.push((Token::Equals, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'=' && bytes[i + 1] == b'>' {
            tokens.push((Token::FatArrow, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'!' && bytes[i + 1] == b'=' {
            tokens.push((Token::NotEquals, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'|' && bytes[i + 1] == b'|' {
            tokens.push((Token::Or, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'&' && bytes[i + 1] == b'&' {
            tokens.push((Token::And, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'<' && bytes[i + 1] == b'=' {
            tokens.push((Token::LessEqual, i..i + 2));
            i += 2;
            continue;
        }
        if i + 1 < len && bytes[i] == b'>' && bytes[i + 1] == b'=' {
            tokens.push((Token::GreaterEqual, i..i + 2));
            i += 2;
            continue;
        }

        // Single char tokens
        let single = match ch {
            b'|' => Token::Pipe,
            b':' => Token::Colon,
            b';' => Token::Semicolon,
            b'=' => Token::Assign,
            b'!' => Token::Bang,
            b'<' => Token::Less,
            b'>' => Token::Greater,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Star,
            b'/' => Token::Slash,
            b'%' => Token::Percent,
            b'&' => Token::Ampersand,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b',' => Token::Comma,
            b'.' => Token::Dot,
            b'?' => Token::Question,
            b'#' => Token::Hash,
            _ => Token::Error,
        };
        tokens.push((single, i..i + 1));
        i += 1;
    }

    tokens
}

fn is_ident_start(ch: u8) -> bool {
    (ch >= b'a' && ch <= b'z') || (ch >= b'A' && ch <= b'Z') || ch == b'_'
}

fn is_ident_continue(ch: u8) -> bool {
    is_ident_start(ch) || (ch >= b'0' && ch <= b'9')
}

fn keyword_or_ident(text: &str) -> Token {
    match text {
        "fn" => Token::KwFn,
        "let" => Token::KwLet,
        "return" => Token::KwReturn,
        "if" => Token::KwIf,
        "else" => Token::KwElse,
        "while" => Token::KwWhile,
        "for" => Token::KwFor,
        "in" => Token::KwIn,
        "match" => Token::KwMatch,
        "impl" => Token::KwImpl,
        "trait" => Token::KwTrait,
        "where" => Token::KwWhere,
        "const" => Token::KwConst,
        "static" => Token::KwStatic,
        "use" => Token::KwUse,
        "mod" => Token::KwMod,
        "pub" => Token::KwPub,
        "async" => Token::KwAsync,
        "await" => Token::KwAwait,
        "struct" => Token::KwStruct,
        "enum" => Token::KwEnum,
        "type" => Token::KwType,
        "extern" => Token::KwExtern,
        "import" => Token::KwImport,
        "int" => Token::TypeInt,
        "bool" => Token::TypeBool,
        "string" => Token::TypeString,
        "void" => Token::TypeVoid,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Identifier(text.to_string()),
    }
}