use crate::token::{Span, SpannedToken, Token};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            let ch = self.input[self.position];
            self.position += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' {
                // Check if next is /
                let pos = self.position;
                if pos + 1 < self.input.len() && self.input[pos + 1] == '/' {
                    // It is a comment
                    self.advance(); // /
                    self.advance(); // /
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> i64 {
        let mut num_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                num_str.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        num_str.parse().unwrap_or(0)
    }

    fn read_string(&mut self) -> String {
        self.advance(); // Skip opening quote
        let mut string_content = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // Skip closing quote
                break;
            }
            string_content.push(self.advance().unwrap());
        }
        string_content
    }

    pub fn next_token(&mut self) -> SpannedToken {
        self.skip_whitespace();

        let span = Span {
            line: self.line,
            column: self.column,
        };

        let token = match self.peek() {
            Some(ch) => match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "fn" => Token::Fn,
                        "let" => Token::Let,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        "extern" => Token::Extern,
                        "struct" => Token::Struct,
                        "while" => Token::While,
                        "for" => Token::For,
                        "true" => Token::True,
                        "false" => Token::False,
                        _ => Token::Identifier(ident),
                    }
                }
                '0'..='9' => Token::Integer(self.read_number()),
                '"' => Token::StringLiteral(self.read_string()),
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '-' => {
                    self.advance();
                    Token::Minus
                }
                '*' => {
                    self.advance();
                    Token::Asterisk
                }
                '/' => {
                    self.advance();
                    Token::Slash
                }
                '.' => {
                    self.advance();
                    Token::Dot
                }
                '=' => {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::Equals
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    self.advance();
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::NotEquals
                    } else {
                        Token::Illegal('!')
                    }
                }
                '<' => {
                    self.advance();
                    Token::LessThan
                }
                '>' => {
                    self.advance();
                    Token::GreaterThan
                }
                '(' => {
                    self.advance();
                    Token::LPren
                }
                ')' => {
                    self.advance();
                    Token::RPren
                }
                '{' => {
                    self.advance();
                    Token::LBrace
                }
                '}' => {
                    self.advance();
                    Token::RBrace
                }
                ';' => {
                    self.advance();
                    Token::SemiColon
                }
                ',' => {
                    self.advance();
                    Token::Comma
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                c => {
                    self.advance();
                    Token::Illegal(c)
                }
            },
            None => Token::EOF,
        };

        SpannedToken { token, span }
    }
}
