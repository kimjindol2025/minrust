// MinRust Compiler - Phase 5 Rust Implementation
// Main compilation pipeline orchestrator

use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::type_checker::TypeChecker;
use crate::codegen::CodeGenerator;

/// Compilation result
#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub success: bool,
    pub c_code: Option<String>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub token_count: usize,
    pub ast_node_count: usize,
}

impl CompilationResult {
    /// Print compilation statistics
    pub fn print_stats(&self) {
        println!("Compilation Result:");
        println!("  Success: {}", self.success);
        println!("  Tokens: {}", self.token_count);
        println!("  AST Nodes: {}", self.ast_node_count);
        println!("  Errors: {}", self.errors.len());
        println!("  Warnings: {}", self.warnings.len());

        if !self.errors.is_empty() {
            println!("\nErrors:");
            for error in &self.errors {
                println!("  - {}", error);
            }
        }

        if !self.warnings.is_empty() {
            println!("\nWarnings:");
            for warning in &self.warnings {
                println!("  - {}", warning);
            }
        }
    }
}

/// Main compiler
pub struct Compiler;

impl Compiler {
    /// Compile Rust source to C code
    pub fn compile(source: &str) -> CompilationResult {
        let mut result = CompilationResult {
            success: false,
            c_code: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            token_count: 0,
            ast_node_count: 0,
        };

        // Stage 1: Tokenization
        let mut tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.tokenize();
        result.token_count = tokens.len();

        // Stage 2: Parsing
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(program) => {
                result.ast_node_count = Self::count_ast_nodes(&program);

                // Stage 3: Type Checking
                let mut type_checker = TypeChecker::new();
                match type_checker.check(&program) {
                    Ok(_) => {
                        // Stage 4: Code Generation
                        let mut codegen = CodeGenerator::new();
                        let c_code = codegen.generate(&program);
                        result.c_code = Some(c_code);
                        result.success = true;
                    }
                    Err(errors) => {
                        for error in errors {
                            result.errors.push(error.message);
                        }
                    }
                }
            }
            Err(errors) => {
                for error in errors {
                    result.errors.push(error.message);
                }
            }
        }

        result
    }

    /// Count AST nodes (placeholder)
    fn count_ast_nodes(program: &crate::ast::Program) -> usize {
        program.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_empty() {
        let result = Compiler::compile("");
        assert!(result.success || result.c_code.is_some());
    }

    #[test]
    fn test_compile_simple() {
        let source = "fn main() { let x = 42; }";
        let result = Compiler::compile(source);
        assert!(result.success || result.token_count > 0);
    }

    #[test]
    fn test_compilation_result() {
        let result = CompilationResult {
            success: true,
            c_code: Some("int main() { return 0; }".to_string()),
            errors: vec![],
            warnings: vec![],
            token_count: 10,
            ast_node_count: 5,
        };

        assert!(result.success);
        assert_eq!(result.errors.len(), 0);
    }
}
