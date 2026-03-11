// MinRust Parser - Phase 5 Rust Implementation (Skeleton)
// Recursive Descent Parser - Full implementation in Phase 5.2

use crate::ast::*;
use crate::tokenizer::Token;

/// Parser error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

/// Parser structure
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    /// Create new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    /// Parse tokens into program
    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>> {
        // TODO: Implement full recursive descent parser
        // For now, return empty program
        Ok(Program { items: Vec::new() })
    }

    /// Check if current token matches expected
    fn check_token(&self, expected: &Token) -> bool {
        if let Some(token) = self.current() {
            std::mem::discriminant(token) == std::mem::discriminant(expected)
        } else {
            false
        }
    }

    /// Get current token
    fn current(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    /// Advance to next token
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    /// Report error
    fn error(&mut self, message: String) {
        self.errors.push(ParseError {
            message,
            line: 0,
            column: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let tokens = vec![Token::Eof];
        let parser = Parser::new(tokens);
        assert_eq!(parser.pos, 0);
    }

    #[test]
    fn test_empty_parse() {
        let mut parser = Parser::new(vec![Token::Eof]);
        let result = parser.parse();
        assert!(result.is_ok());
    }
}
