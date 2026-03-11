// MinRust Tokenizer - Phase 5 Rust Implementation
// Complete rewrite of Julia tokenizer in Rust

use std::collections::HashMap;

/// Token types - All possible token variants
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Keyword(String),

    // Identifiers & Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLit(String),
    CharLit(char),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Ampersand,
    Pipe,
    Caret,
    Tilde,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    Dot,
    DotDot,
    Arrow,
    FatArrow,
    Question,

    // Special
    Eof,
}

/// Token with location information
#[derive(Debug, Clone)]
pub struct TokenWithLocation {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

/// Tokenizer structure
pub struct Tokenizer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    keywords: HashMap<String, bool>,
}

impl Tokenizer {
    /// Create new tokenizer from input string
    pub fn new(input: &str) -> Self {
        let mut keywords = HashMap::new();

        // Add all Rust keywords
        for keyword in &[
            "fn", "let", "const", "mut", "if", "else", "for", "while", "loop", "match",
            "return", "break", "continue", "struct", "impl", "trait", "pub", "use", "mod",
            "crate", "as", "true", "false", "unsafe", "async", "await", "where", "in",
            "ref", "extern", "type", "enum", "union", "self", "Self",
        ] {
            keywords.insert(keyword.to_string(), true);
        }

        Tokenizer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            keywords,
        }
    }

    /// Get current character
    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    /// Peek ahead by offset
    fn peek(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.input.len() {
            Some(self.input[self.pos + offset])
        } else {
            None
        }
    }

    /// Move to next character
    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.current() {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    /// Check if character is whitespace
    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    /// Check if character is digit
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    /// Check if character is alpha
    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    /// Check if character is alphanumeric
    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    /// Tokenize entire input
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            // Skip whitespace
            while let Some(c) = self.current() {
                if Self::is_whitespace(c) {
                    self.advance();
                } else {
                    break;
                }
            }

            match self.current() {
                None => {
                    tokens.push(Token::Eof);
                    break;
                }

                // Comments
                Some('/') => {
                    self.advance();
                    if self.current() == Some('/') {
                        // Line comment
                        while self.current() != Some('\n') && self.current() != None {
                            self.advance();
                        }
                    } else if self.current() == Some('*') {
                        // Block comment
                        self.advance();
                        while self.current() != None {
                            if self.current() == Some('*') && self.peek(1) == Some('/') {
                                self.advance();
                                self.advance();
                                break;
                            }
                            self.advance();
                        }
                    } else if self.current() == Some('=') {
                        self.advance();
                        tokens.push(Token::Slash); // Simplified: /= not supported
                    } else {
                        tokens.push(Token::Slash);
                    }
                }

                // Parentheses
                Some('(') => {
                    self.advance();
                    tokens.push(Token::LeftParen);
                }
                Some(')') => {
                    self.advance();
                    tokens.push(Token::RightParen);
                }

                // Braces
                Some('{') => {
                    self.advance();
                    tokens.push(Token::LeftBrace);
                }
                Some('}') => {
                    self.advance();
                    tokens.push(Token::RightBrace);
                }

                // Brackets
                Some('[') => {
                    self.advance();
                    tokens.push(Token::LeftBracket);
                }
                Some(']') => {
                    self.advance();
                    tokens.push(Token::RightBracket);
                }

                // Punctuation
                Some(',') => {
                    self.advance();
                    tokens.push(Token::Comma);
                }
                Some(';') => {
                    self.advance();
                    tokens.push(Token::Semicolon);
                }
                Some(':') => {
                    self.advance();
                    if self.current() == Some(':') {
                        self.advance();
                        tokens.push(Token::DoubleColon);
                    } else {
                        tokens.push(Token::Colon);
                    }
                }
                Some('.') => {
                    self.advance();
                    if self.current() == Some('.') {
                        self.advance();
                        tokens.push(Token::DotDot);
                    } else {
                        tokens.push(Token::Dot);
                    }
                }

                // Operators
                Some('+') => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                Some('-') => {
                    self.advance();
                    if self.current() == Some('>') {
                        self.advance();
                        tokens.push(Token::Arrow);
                    } else {
                        tokens.push(Token::Minus);
                    }
                }
                Some('*') => {
                    self.advance();
                    tokens.push(Token::Star);
                }
                Some('%') => {
                    self.advance();
                    tokens.push(Token::Percent);
                }
                Some('=') => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        tokens.push(Token::Equal);
                    } else if self.current() == Some('>') {
                        self.advance();
                        tokens.push(Token::FatArrow);
                    } else {
                        tokens.push(Token::Assign);
                    }
                }
                Some('!') => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        tokens.push(Token::NotEqual);
                    } else {
                        tokens.push(Token::Not);
                    }
                }
                Some('<') => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        tokens.push(Token::LessEqual);
                    } else if self.current() == Some('<') {
                        self.advance();
                        tokens.push(Token::Or); // Simplified
                    } else {
                        tokens.push(Token::Less);
                    }
                }
                Some('>') => {
                    self.advance();
                    if self.current() == Some('=') {
                        self.advance();
                        tokens.push(Token::GreaterEqual);
                    } else if self.current() == Some('>') {
                        self.advance();
                        tokens.push(Token::Or); // Simplified
                    } else {
                        tokens.push(Token::Greater);
                    }
                }
                Some('&') => {
                    self.advance();
                    if self.current() == Some('&') {
                        self.advance();
                        tokens.push(Token::And);
                    } else {
                        tokens.push(Token::Ampersand);
                    }
                }
                Some('|') => {
                    self.advance();
                    if self.current() == Some('|') {
                        self.advance();
                        tokens.push(Token::Or);
                    } else {
                        tokens.push(Token::Pipe);
                    }
                }
                Some('^') => {
                    self.advance();
                    tokens.push(Token::Caret);
                }
                Some('~') => {
                    self.advance();
                    tokens.push(Token::Tilde);
                }
                Some('?') => {
                    self.advance();
                    tokens.push(Token::Question);
                }

                // String literals
                Some('"') => {
                    self.advance();
                    let mut string = String::new();
                    while self.current() != Some('"') && self.current() != None {
                        if self.current() == Some('\\') {
                            self.advance();
                            match self.current() {
                                Some('n') => string.push('\n'),
                                Some('t') => string.push('\t'),
                                Some('\\') => string.push('\\'),
                                Some('"') => string.push('"'),
                                Some(c) => string.push(c),
                                None => break,
                            }
                            self.advance();
                        } else {
                            if let Some(c) = self.current() {
                                string.push(c);
                            }
                            self.advance();
                        }
                    }
                    if self.current() == Some('"') {
                        self.advance();
                    }
                    tokens.push(Token::StringLit(string));
                }

                // Character literals
                Some('\'') => {
                    self.advance();
                    let mut ch = ' ';
                    if self.current() == Some('\\') {
                        self.advance();
                        ch = match self.current() {
                            Some('n') => '\n',
                            Some('t') => '\t',
                            Some('\\') => '\\',
                            Some('\'') => '\'',
                            Some(c) => c,
                            None => ' ',
                        };
                        self.advance();
                    } else if let Some(c) = self.current() {
                        ch = c;
                        self.advance();
                    }
                    if self.current() == Some('\'') {
                        self.advance();
                    }
                    tokens.push(Token::CharLit(ch));
                }

                // Numbers
                Some(c) if Self::is_digit(c) => {
                    let mut number = String::new();
                    while let Some(d) = self.current() {
                        if Self::is_digit(d) {
                            number.push(d);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if self.current() == Some('.') && self.peek(1).map_or(false, Self::is_digit) {
                        number.push('.');
                        self.advance();
                        while let Some(d) = self.current() {
                            if Self::is_digit(d) {
                                number.push(d);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::Float(number.parse().unwrap_or(0.0)));
                    } else {
                        tokens.push(Token::Integer(number.parse().unwrap_or(0)));
                    }
                }

                // Identifiers and keywords
                Some(c) if Self::is_alpha(c) => {
                    let mut ident = String::new();
                    while let Some(ch) = self.current() {
                        if Self::is_alphanumeric(ch) {
                            ident.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if self.keywords.contains_key(&ident) {
                        tokens.push(Token::Keyword(ident));
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }

                // Unknown character - skip
                Some(_) => {
                    self.advance();
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut tokenizer = Tokenizer::new("let x = 42;");
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 6); // let, x, =, 42, ;, EOF
        assert_eq!(tokens[0], Token::Keyword("let".to_string()));
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Assign);
        assert_eq!(tokens[3], Token::Integer(42));
        assert_eq!(tokens[4], Token::Semicolon);
    }

    #[test]
    fn test_function_declaration() {
        let mut tokenizer = Tokenizer::new("fn add(a, b) { }");
        let tokens = tokenizer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t, Token::Keyword(s) if s == "fn")));
        assert!(tokens.iter().any(|t| matches!(t, Token::Identifier(s) if s == "add")));
    }

    #[test]
    fn test_operators() {
        let mut tokenizer = Tokenizer::new("+ - * / == !=");
        let tokens = tokenizer.tokenize();

        assert!(tokens.contains(&Token::Plus));
        assert!(tokens.contains(&Token::Minus));
        assert!(tokens.contains(&Token::Star));
        assert!(tokens.contains(&Token::Slash));
        assert!(tokens.contains(&Token::Equal));
        assert!(tokens.contains(&Token::NotEqual));
    }

    #[test]
    fn test_string_literal() {
        let mut tokenizer = Tokenizer::new(r#""hello""#);
        let tokens = tokenizer.tokenize();

        assert!(tokens.iter().any(|t| matches!(t, Token::StringLit(s) if s == "hello")));
    }

    #[test]
    fn test_comments() {
        let mut tokenizer = Tokenizer::new("let x = 42; // comment");
        let tokens = tokenizer.tokenize();

        // Comment should be skipped
        assert_eq!(tokens.len(), 6); // let, x, =, 42, ;, EOF (no comment token)
    }
}
