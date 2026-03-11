// MinRust Parser - Phase 5 Rust Implementation (완성)
// Recursive Descent Parser with Operator Precedence

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
        let mut items = Vec::new();

        while !self.is_at_end() {
            if let Ok(item) = self.parse_item() {
                items.push(item);
            } else {
                self.synchronize();
            }
        }

        if self.errors.is_empty() {
            Ok(Program { items })
        } else {
            Err(self.errors.clone())
        }
    }

    // ============================================================
    // Top-level Items
    // ============================================================

    fn parse_item(&mut self) -> Result<Item, ParseError> {
        match self.current() {
            Some(Token::Keyword(kw)) if kw == "fn" => Ok(Item::Function(self.parse_fn_decl()?)),
            Some(Token::Keyword(kw)) if kw == "struct" => Ok(Item::Struct(self.parse_struct_decl()?)),
            Some(Token::Keyword(kw)) if kw == "impl" => Ok(Item::ImplBlock(self.parse_impl_block()?)),
            _ => Err(self.error_current("Expected fn, struct, or impl")),
        }
    }

    fn parse_fn_decl(&mut self) -> Result<FnDecl, ParseError> {
        self.consume(Token::Keyword("fn".to_string()), "Expected 'fn'")?;

        let name = self.parse_identifier()?;

        self.consume(Token::LeftParen, "Expected '('")?;
        let params = self.parse_params()?;
        self.consume(Token::RightParen, "Expected ')'")?;

        let return_type = if self.check(Token::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(Token::LeftBrace, "Expected '{'")?;
        let body = self.parse_block()?;
        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(FnDecl {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_struct_decl(&mut self) -> Result<StructDecl, ParseError> {
        self.consume(Token::Keyword("struct".to_string()), "Expected 'struct'")?;

        let name = self.parse_identifier()?;

        self.consume(Token::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        while !self.check(Token::RightBrace) && !self.is_at_end() {
            let field_name = self.parse_identifier()?;
            self.consume(Token::Colon, "Expected ':'")?;
            let ty = self.parse_type()?;
            fields.push(StructField { name: field_name, ty });

            if self.check(Token::Comma) {
                self.advance();
            }
        }

        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(StructDecl { name, fields })
    }

    fn parse_impl_block(&mut self) -> Result<ImplBlock, ParseError> {
        self.consume(Token::Keyword("impl".to_string()), "Expected 'impl'")?;

        let target = self.parse_identifier()?;

        self.consume(Token::LeftBrace, "Expected '{'")?;

        let mut methods = Vec::new();
        while !self.check(Token::RightBrace) && !self.is_at_end() {
            if let Ok(method) = self.parse_fn_decl() {
                methods.push(method);
            } else {
                self.synchronize();
            }
        }

        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(ImplBlock { target, methods })
    }

    // ============================================================
    // Statements
    // ============================================================

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();

        while !self.check(Token::RightBrace) && !self.is_at_end() {
            if let Ok(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.synchronize();
            }
        }

        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.current() {
            Some(Token::Keyword(kw)) if kw == "let" => self.parse_var_decl(),
            Some(Token::Keyword(kw)) if kw == "const" => self.parse_const_decl(),
            Some(Token::Keyword(kw)) if kw == "if" => self.parse_if_stmt(),
            Some(Token::Keyword(kw)) if kw == "for" => self.parse_for_stmt(),
            Some(Token::Keyword(kw)) if kw == "while" => self.parse_while_stmt(),
            Some(Token::Keyword(kw)) if kw == "loop" => self.parse_loop_stmt(),
            Some(Token::Keyword(kw)) if kw == "match" => self.parse_match_stmt(),
            Some(Token::Keyword(kw)) if kw == "return" => self.parse_return_stmt(),
            Some(Token::LeftBrace) => {
                self.advance();
                let stmts = self.parse_block()?;
                self.consume(Token::RightBrace, "Expected '}'")?;
                Ok(Stmt::Block(stmts))
            }
            _ => {
                let expr = self.parse_expr()?;
                if self.check(Token::Semicolon) {
                    self.advance();
                }
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("let".to_string()), "Expected 'let'")?;

        let mutable = if self.check(Token::Keyword("mut".to_string())) {
            self.advance();
            true
        } else {
            false
        };

        let name = self.parse_identifier()?;

        let ty = if self.check(Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let init = if self.check(Token::Assign) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        if self.check(Token::Semicolon) {
            self.advance();
        }

        Ok(Stmt::VarDecl {
            name,
            ty,
            init,
            mutable,
        })
    }

    fn parse_const_decl(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("const".to_string()), "Expected 'const'")?;

        let name = self.parse_identifier()?;

        let ty = if self.check(Token::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(Token::Assign, "Expected '='")?;
        let value = self.parse_expr()?;

        if self.check(Token::Semicolon) {
            self.advance();
        }

        Ok(Stmt::ConstDecl { name, ty, value })
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("if".to_string()), "Expected 'if'")?;

        let condition = self.parse_expr()?;

        self.consume(Token::LeftBrace, "Expected '{'")?;
        let then_body = self.parse_block()?;
        self.consume(Token::RightBrace, "Expected '}'")?;

        let else_body = if self.check(Token::Keyword("else".to_string())) {
            self.advance();
            self.consume(Token::LeftBrace, "Expected '{'")?;
            let body = self.parse_block()?;
            self.consume(Token::RightBrace, "Expected '}'")?;
            Some(body)
        } else {
            None
        };

        Ok(Stmt::IfStmt {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("for".to_string()), "Expected 'for'")?;

        let var = self.parse_identifier()?;

        self.consume(Token::Keyword("in".to_string()), "Expected 'in'")?;

        let iter = self.parse_expr()?;

        self.consume(Token::LeftBrace, "Expected '{'")?;
        let body = self.parse_block()?;
        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(Stmt::ForStmt { var, iter, body })
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("while".to_string()), "Expected 'while'")?;

        let condition = self.parse_expr()?;

        self.consume(Token::LeftBrace, "Expected '{'")?;
        let body = self.parse_block()?;
        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(Stmt::WhileStmt { condition, body })
    }

    fn parse_loop_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("loop".to_string()), "Expected 'loop'")?;

        self.consume(Token::LeftBrace, "Expected '{'")?;
        let body = self.parse_block()?;
        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(Stmt::LoopStmt { body })
    }

    fn parse_match_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("match".to_string()), "Expected 'match'")?;

        let expr = self.parse_expr()?;

        self.consume(Token::LeftBrace, "Expected '{'")?;

        let mut arms = Vec::new();
        while !self.check(Token::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_identifier()?;
            self.consume(Token::FatArrow, "Expected '=>'")?;

            self.consume(Token::LeftBrace, "Expected '{'")?;
            let body = self.parse_block()?;
            self.consume(Token::RightBrace, "Expected '}'")?;

            arms.push((pattern, body));

            if self.check(Token::Comma) {
                self.advance();
            }
        }

        self.consume(Token::RightBrace, "Expected '}'")?;

        Ok(Stmt::MatchStmt { expr, arms })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.consume(Token::Keyword("return".to_string()), "Expected 'return'")?;

        let expr = if self.check(Token::Semicolon) || self.check(Token::RightBrace) {
            None
        } else {
            Some(self.parse_expr()?)
        };

        if self.check(Token::Semicolon) {
            self.advance();
        }

        Ok(Stmt::ReturnStmt(expr))
    }

    // ============================================================
    // Expressions (Precedence Climbing)
    // ============================================================

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_logical_or()?;

        if self.check(Token::Assign) {
            self.advance();
            let right = self.parse_assignment()?;
            return Ok(Expr::BinaryOp {
                left: Box::new(expr),
                op: "=".to_string(),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_logical_and()?;

        while self.check(Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: "||".to_string(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;

        while self.check(Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: "&&".to_string(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = match self.current() {
                Some(Token::Equal) => "==",
                Some(Token::NotEqual) => "!=",
                _ => break,
            };

            self.advance();
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_additive()?;

        loop {
            let op = match self.current() {
                Some(Token::Less) => "<",
                Some(Token::LessEqual) => "<=",
                Some(Token::Greater) => ">",
                Some(Token::GreaterEqual) => ">=",
                _ => break,
            };

            self.advance();
            let right = self.parse_additive()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match self.current() {
                Some(Token::Plus) => "+",
                Some(Token::Minus) => "-",
                _ => break,
            };

            self.advance();
            let right = self.parse_multiplicative()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = match self.current() {
                Some(Token::Star) => "*",
                Some(Token::Slash) => "/",
                Some(Token::Percent) => "%",
                _ => break,
            };

            self.advance();
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op: op.to_string(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.current() {
            Some(Token::Minus) => {
                self.advance();
                Ok(Expr::UnaryOp {
                    op: "-".to_string(),
                    operand: Box::new(self.parse_unary()?),
                })
            }
            Some(Token::Not) => {
                self.advance();
                Ok(Expr::UnaryOp {
                    op: "!".to_string(),
                    operand: Box::new(self.parse_unary()?),
                })
            }
            Some(Token::Ampersand) => {
                self.advance();
                let mutable = if self.check(Token::Keyword("mut".to_string())) {
                    self.advance();
                    true
                } else {
                    false
                };
                Ok(Expr::UnaryOp {
                    op: if mutable { "&mut" } else { "&" }.to_string(),
                    operand: Box::new(self.parse_unary()?),
                })
            }
            Some(Token::Star) => {
                self.advance();
                Ok(Expr::UnaryOp {
                    op: "*".to_string(),
                    operand: Box::new(self.parse_unary()?),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current() {
                Some(Token::LeftParen) => {
                    self.advance();
                    let args = self.parse_args()?;
                    self.consume(Token::RightParen, "Expected ')'")?;

                    match expr {
                        Expr::Identifier(name) => {
                            expr = Expr::Call {
                                func: name,
                                args,
                            };
                        }
                        _ => {
                            return Err(self.error_current("Invalid function call"));
                        }
                    }
                }
                Some(Token::LeftBracket) => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.consume(Token::RightBracket, "Expected ']'")?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Some(Token::Dot) => {
                    self.advance();
                    let field = self.parse_identifier()?;
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field,
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.current() {
            Some(Token::Integer(n)) => {
                let n = *n;
                self.advance();
                Ok(Expr::Literal(Literal::Integer(n)))
            }
            Some(Token::Float(f)) => {
                let f = *f;
                self.advance();
                Ok(Expr::Literal(Literal::Float(f)))
            }
            Some(Token::StringLit(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Some(Token::CharLit(c)) => {
                let c = *c;
                self.advance();
                Ok(Expr::Literal(Literal::Char(c)))
            }
            Some(Token::Keyword(kw)) if kw == "true" => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true)))
            }
            Some(Token::Keyword(kw)) if kw == "false" => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false)))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            Some(Token::LeftParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.consume(Token::RightParen, "Expected ')'")?;
                Ok(expr)
            }
            Some(Token::LeftBracket) => {
                self.advance();
                let mut elements = Vec::new();
                while !self.check(Token::RightBracket) && !self.is_at_end() {
                    elements.push(self.parse_expr()?);
                    if self.check(Token::Comma) {
                        self.advance();
                    }
                }
                self.consume(Token::RightBracket, "Expected ']'")?;
                Ok(Expr::ArrayLiteral(elements))
            }
            _ => Err(self.error_current("Expected expression")),
        }
    }

    // ============================================================
    // Types
    // ============================================================

    fn parse_type(&mut self) -> Result<RustType, ParseError> {
        match self.current() {
            Some(Token::Ampersand) => {
                self.advance();
                let mutable = if self.check(Token::Keyword("mut".to_string())) {
                    self.advance();
                    true
                } else {
                    false
                };
                let inner = Box::new(self.parse_type()?);
                Ok(RustType::Reference { inner, mutable })
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(RustType::Primitive(name))
            }
            _ => Err(self.error_current("Expected type")),
        }
    }

    // ============================================================
    // Helper Functions
    // ============================================================

    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();

        while !self.check(Token::RightParen) && !self.is_at_end() {
            let name = self.parse_identifier()?;
            self.consume(Token::Colon, "Expected ':'")?;
            let ty = self.parse_type()?;
            params.push(Param { name, ty });

            if self.check(Token::Comma) {
                self.advance();
            }
        }

        Ok(params)
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        while !self.check(Token::RightParen) && !self.is_at_end() {
            args.push(self.parse_expr()?);
            if self.check(Token::Comma) {
                self.advance();
            }
        }

        Ok(args)
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        match self.current() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(self.error_current("Expected identifier")),
        }
    }

    fn check(&self, token: Token) -> bool {
        if let Some(current) = self.current() {
            std::mem::discriminant(current) == std::mem::discriminant(&token)
        } else {
            false
        }
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<(), ParseError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(self.error_current(message))
        }
    }

    fn current(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current(), Some(Token::Eof) | None)
    }

    fn error_current(&mut self, message: &str) -> ParseError {
        self.error(message.to_string());
        ParseError {
            message: message.to_string(),
            line: 0,
            column: 0,
        }
    }

    fn error(&mut self, message: String) {
        self.errors.push(ParseError {
            message,
            line: 0,
            column: 0,
        });
    }

    fn synchronize(&mut self) {
        while !self.is_at_end() {
            match self.current() {
                Some(Token::Semicolon) => {
                    self.advance();
                    break;
                }
                Some(Token::Keyword(_)) => break,
                _ => self.advance(),
            }
        }
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

    #[test]
    fn test_parse_simple_function() {
        let tokens = vec![
            Token::Keyword("fn".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Eof,
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());

        if let Ok(program) = result {
            assert_eq!(program.items.len(), 1);
        }
    }

    #[test]
    fn test_parse_literal_expression() {
        let tokens = vec![
            Token::Integer(42),
            Token::Eof,
        ];

        let mut parser = Parser::new(tokens);
        if let Ok(expr) = parser.parse_primary() {
            match expr {
                Expr::Literal(Literal::Integer(42)) => assert!(true),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_binary_op() {
        let tokens = vec![
            Token::Integer(10),
            Token::Plus,
            Token::Integer(20),
            Token::Eof,
        ];

        let mut parser = Parser::new(tokens);
        if let Ok(expr) = parser.parse_expr() {
            match expr {
                Expr::BinaryOp {
                    op,
                    ..
                } => assert_eq!(op, "+"),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_parse_identifiers() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Eof,
        ];

        let mut parser = Parser::new(tokens);
        if let Ok(expr) = parser.parse_primary() {
            match expr {
                Expr::Identifier(name) => assert_eq!(name, "x"),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_operator_precedence() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let tokens = vec![
            Token::Integer(2),
            Token::Plus,
            Token::Integer(3),
            Token::Star,
            Token::Integer(4),
            Token::Eof,
        ];

        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr().unwrap();

        // Verify: the top-level operator is +
        match expr {
            Expr::BinaryOp { op, .. } => assert_eq!(op, "+"),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_string_literal() {
        let tokens = vec![
            Token::StringLit("hello".to_string()),
            Token::Eof,
        ];

        let mut parser = Parser::new(tokens);
        if let Ok(expr) = parser.parse_primary() {
            match expr {
                Expr::Literal(Literal::String(s)) => assert_eq!(s, "hello"),
                _ => assert!(false),
            }
        }
    }
}
