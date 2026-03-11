// MinRust Simple Tokenizer - Rust 구현 (Phase 3: 자체호스팅)
// Julia 컴파일러가 생성한 C 코드로부터 동작하는 토크나이저

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // 키워드
    Keyword(String),

    // 식별자 & 리터럴
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLit(String),
    CharLit(char),

    // 연산자 & 펑크추에이션
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

    // 펑크추에이션
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

    // 특수
    Eof,
}

pub struct Tokenizer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn current(&self) -> Option<char> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.input.len() {
            Some(self.input[self.pos + offset])
        } else {
            None
        }
    }

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

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            // 공백 스킵
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
                Some('/') => {
                    self.advance();
                    if self.current() == Some('/') {
                        // 라인 주석
                        while self.current() != Some('\n') && self.current() != None {
                            self.advance();
                        }
                    } else if self.current() == Some('*') {
                        // 블록 주석
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
                        tokens.push(Token::Slash);  // 간단화: /=는 미지원
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                Some('(') => {
                    self.advance();
                    tokens.push(Token::LeftParen);
                }
                Some(')') => {
                    self.advance();
                    tokens.push(Token::RightParen);
                }
                Some('{') => {
                    self.advance();
                    tokens.push(Token::LeftBrace);
                }
                Some('}') => {
                    self.advance();
                    tokens.push(Token::RightBrace);
                }
                Some('[') => {
                    self.advance();
                    tokens.push(Token::LeftBracket);
                }
                Some(']') => {
                    self.advance();
                    tokens.push(Token::RightBracket);
                }
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
                        tokens.push(Token::Or);  // 간단화
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
                        tokens.push(Token::Or);  // 간단화
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

                    // 키워드 체크
                    let token = match ident.as_str() {
                        "fn" | "let" | "const" | "mut" | "if" | "else" | "for" | "while" |
                        "loop" | "match" | "return" | "break" | "continue" | "struct" |
                        "impl" | "trait" | "pub" | "use" | "mod" | "crate" | "as" |
                        "true" | "false" | "unsafe" | "async" | "await" => Token::Keyword(ident),
                        _ => Token::Identifier(ident),
                    };
                    tokens.push(token);
                }
                Some(_) => {
                    self.advance();
                }
            }
        }

        tokens
    }
}

// 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens() {
        let mut tokenizer = Tokenizer::new("let x = 42;");
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 6);  // let, x, =, 42, ;, EOF
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
}

fn main() {
    let code = "let x = 42; fn main() { }";
    let mut tokenizer = Tokenizer::new(code);
    let tokens = tokenizer.tokenize();

    println!("Tokens: {:?}", tokens);
}
