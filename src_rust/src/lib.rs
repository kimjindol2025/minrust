// MinRust Compiler Library - Phase 5 Rust Self-Hosting Implementation
// Complete rewrite of Julia compiler in Rust

pub mod tokenizer;
pub mod ast;
pub mod parser;
pub mod type_checker;
pub mod codegen;
pub mod compiler;

// Re-export main public API
pub use compiler::{Compiler, CompilationResult};
pub use tokenizer::Token;
pub use ast::Program;

/// MinRust Compiler version
pub const VERSION: &str = "0.3.0";

/// Compile Rust subset code to C code
pub fn compile(source: &str) -> CompilationResult {
    Compiler::compile(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.3.0");
    }

    #[test]
    fn test_simple_compilation() {
        let source = r#"
            fn main() {
                let x = 42;
            }
        "#;

        let result = compile(source);
        assert!(result.success);
        assert!(result.c_code.is_some());
    }
}
