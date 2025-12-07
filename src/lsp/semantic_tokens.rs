#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokensLegend, Url,
};

/// Semantic tokens provider for syntax highlighting
pub struct SemanticTokensProvider {
    legend: SemanticTokensLegend,
    token_cache: HashMap<String, Vec<SemanticToken>>,
}

impl SemanticTokensProvider {
    pub fn new() -> Self {
        SemanticTokensProvider {
            legend: Self::create_legend(),
            token_cache: HashMap::new(),
        }
    }

    fn create_legend() -> SemanticTokensLegend {
        SemanticTokensLegend {
            token_types: vec![
                SemanticTokenType::NAMESPACE,
                SemanticTokenType::TYPE,
                SemanticTokenType::CLASS,
                SemanticTokenType::ENUM,
                SemanticTokenType::INTERFACE,
                SemanticTokenType::STRUCT,
                SemanticTokenType::TYPE_PARAMETER,
                SemanticTokenType::PARAMETER,
                SemanticTokenType::VARIABLE,
                SemanticTokenType::PROPERTY,
                SemanticTokenType::ENUM_MEMBER,
                SemanticTokenType::EVENT,
                SemanticTokenType::FUNCTION,
                SemanticTokenType::METHOD,
                SemanticTokenType::MACRO,
                SemanticTokenType::KEYWORD,
                SemanticTokenType::MODIFIER,
                SemanticTokenType::COMMENT,
                SemanticTokenType::STRING,
                SemanticTokenType::NUMBER,
                SemanticTokenType::REGEXP,
                SemanticTokenType::OPERATOR,
            ],
            token_modifiers: vec![
                SemanticTokenModifier::DECLARATION,
                SemanticTokenModifier::DEFINITION,
                SemanticTokenModifier::READONLY,
                SemanticTokenModifier::STATIC,
                SemanticTokenModifier::DEPRECATED,
                SemanticTokenModifier::ABSTRACT,
                SemanticTokenModifier::ASYNC,
                SemanticTokenModifier::MODIFICATION,
                SemanticTokenModifier::DOCUMENTATION,
                SemanticTokenModifier::DEFAULT_LIBRARY,
            ],
        }
    }

    pub fn legend(&self) -> &SemanticTokensLegend {
        &self.legend
    }

    /// Generate semantic tokens for a document
    pub fn provide_tokens(&mut self, uri: &Url, text: &str) -> Vec<SemanticToken> {
        // Check cache first
        if let Some(cached) = self.token_cache.get(uri.as_str()) {
            return cached.clone();
        }

        let tokens = self.tokenize(text);
        self.token_cache.insert(uri.to_string(), tokens.clone());
        tokens
    }

    /// Tokenize source code
    fn tokenize(&self, text: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let mut line = 0;
        let mut start_char = 0;

        // Simple tokenization (in reality would use full parser)
        for (idx, text_line) in text.lines().enumerate() {
            line = idx as u32;
            let line_tokens = self.tokenize_line(text_line, line);
            tokens.extend(line_tokens);
        }

        // Convert to relative positions (LSP format)
        self.convert_to_relative(tokens)
    }

    /// Tokenize a single line
    fn tokenize_line(&self, line: &str, line_num: u32) -> Vec<AbsoluteToken> {
        let mut tokens = Vec::new();
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut char_offset = 0;

        for word in words {
            // Find actual position in line
            if let Some(pos) = line[char_offset..].find(word) {
                char_offset += pos;

                let token_type = self.classify_token(word);
                let modifier = self.get_modifiers(word);

                tokens.push(AbsoluteToken {
                    line: line_num,
                    start_char: char_offset as u32,
                    length: word.len() as u32,
                    token_type,
                    token_modifier: modifier,
                });

                char_offset += word.len();
            }
        }

        tokens
    }

    /// Classify token type
    fn classify_token(&self, word: &str) -> u32 {
        // Keywords
        if self.is_keyword(word) {
            return 15; // KEYWORD
        }

        // Types (capitalized)
        if word.chars().next().map_or(false, |c| c.is_uppercase()) {
            return 2; // CLASS/TYPE
        }

        // Functions (followed by parenthesis - simplified)
        if word.ends_with("()") || word.contains('(') {
            return 12; // FUNCTION
        }

        // Numbers
        if word.parse::<f64>().is_ok() {
            return 19; // NUMBER
        }

        // Strings (contains quotes)
        if word.contains('"') || word.contains('\'') {
            return 18; // STRING
        }

        // Default to variable
        8 // VARIABLE
    }

    /// Check if word is a keyword
    fn is_keyword(&self, word: &str) -> bool {
        matches!(
            word,
            "fn" | "class"
                | "struct"
                | "enum"
                | "trait"
                | "impl"
                | "pub"
                | "use"
                | "mod"
                | "let"
                | "mut"
                | "const"
                | "if"
                | "else"
                | "while"
                | "for"
                | "loop"
                | "match"
                | "return"
                | "break"
                | "continue"
                | "as"
                | "in"
                | "true"
                | "false"
                | "self"
                | "Self"
        )
    }

    /// Get token modifiers
    fn get_modifiers(&self, word: &str) -> u32 {
        let mut modifiers = 0u32;

        // Check for common patterns
        if word.starts_with('_') {
            modifiers |= 1 << 2; // READONLY
        }

        if word.chars().all(|c| c.is_uppercase() || c == '_') {
            modifiers |= 1 << 3; // STATIC/CONST
        }

        modifiers
    }

    /// Convert absolute positions to relative (LSP format)
    fn convert_to_relative(&self, tokens: Vec<AbsoluteToken>) -> Vec<SemanticToken> {
        let mut result = Vec::new();
        let mut prev_line = 0;
        let mut prev_start = 0;

        for token in tokens {
            let delta_line = token.line - prev_line;
            let delta_start = if delta_line == 0 {
                token.start_char - prev_start
            } else {
                token.start_char
            };

            result.push(SemanticToken {
                delta_line,
                delta_start,
                length: token.length,
                token_type: token.token_type,
                token_modifiers_bitset: token.token_modifier,
            });

            prev_line = token.line;
            prev_start = token.start_char;
        }

        result
    }

    /// Clear cache for a document
    pub fn invalidate_cache(&mut self, uri: &Url) {
        self.token_cache.remove(uri.as_str());
    }

    /// Clear entire cache
    pub fn clear_cache(&mut self) {
        self.token_cache.clear();
    }
}

/// Absolute position token (internal representation)
#[derive(Debug, Clone)]
struct AbsoluteToken {
    line: u32,
    start_char: u32,
    length: u32,
    token_type: u32,
    token_modifier: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_detection() {
        let provider = SemanticTokensProvider::new();

        assert!(provider.is_keyword("fn"));
        assert!(provider.is_keyword("class"));
        assert!(provider.is_keyword("if"));
        assert!(!provider.is_keyword("hello"));
    }

    #[test]
    fn test_token_classification() {
        let provider = SemanticTokensProvider::new();

        assert_eq!(provider.classify_token("fn"), 15); // KEYWORD
        assert_eq!(provider.classify_token("MyClass"), 2); // TYPE
        assert_eq!(provider.classify_token("123"), 19); // NUMBER
    }
}
