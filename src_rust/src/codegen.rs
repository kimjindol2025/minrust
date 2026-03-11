// MinRust Code Generator - Phase 5 Rust Implementation (Skeleton)
// Generates C code from typed AST - Full implementation in Phase 5.4

use crate::ast::*;

/// Code generator
pub struct CodeGenerator {
    output: String,
    var_counter: usize,
    label_counter: usize,
    indent_level: usize,
}

impl CodeGenerator {
    /// Create new code generator
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            var_counter: 0,
            label_counter: 0,
            indent_level: 0,
        }
    }

    /// Generate C code from program
    pub fn generate(&mut self, program: &Program) -> String {
        // TODO: Implement full code generation

        // Add include directives
        self.output.push_str("#include <stdio.h>\n");
        self.output.push_str("#include <stdlib.h>\n");
        self.output.push_str("#include \"minrust_stdlib.h\"\n\n");

        // Add main function stub
        self.output.push_str("int main() {\n");
        self.output.push_str("    return 0;\n");
        self.output.push_str("}\n");

        self.output.clone()
    }

    /// Convert Rust type to C type
    pub fn rust_type_to_c(&self, ty: &RustType) -> String {
        match ty {
            RustType::Primitive(name) => match name.as_str() {
                "i32" => "int".to_string(),
                "i64" => "long".to_string(),
                "f64" => "double".to_string(),
                "bool" => "int".to_string(),
                "char" => "char".to_string(),
                "String" => "char*".to_string(),
                _ => "int".to_string(),
            },
            RustType::Reference { inner, .. } => {
                let inner_c = self.rust_type_to_c(inner);
                format!("{}*", inner_c)
            }
            RustType::Array { element, size } => {
                let elem_c = self.rust_type_to_c(element);
                if let Some(s) = size {
                    format!("{}[{}]", elem_c, s)
                } else {
                    format!("{}*", elem_c)
                }
            }
            RustType::Vec(inner) => {
                let inner_c = self.rust_type_to_c(inner);
                format!("{}*", inner_c)
            }
            _ => "int".to_string(),
        }
    }

    /// Add indent
    fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Reduce indent
    fn unindent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    /// Get indent string
    fn indent_str(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    /// Generate temporary variable name
    fn gen_temp_var(&mut self) -> String {
        let name = format!("tmp_{}", self.var_counter);
        self.var_counter += 1;
        name
    }

    /// Generate label
    fn gen_label(&mut self) -> String {
        let name = format!("L{}", self.label_counter);
        self.label_counter += 1;
        name
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codegen_creation() {
        let gen = CodeGenerator::new();
        assert_eq!(gen.var_counter, 0);
    }

    #[test]
    fn test_type_conversion() {
        let gen = CodeGenerator::new();
        let i32_type = RustType::Primitive("i32".to_string());
        assert_eq!(gen.rust_type_to_c(&i32_type), "int");
    }

    #[test]
    fn test_generate_empty() {
        let mut gen = CodeGenerator::new();
        let program = Program { items: Vec::new() };
        let code = gen.generate(&program);
        assert!(code.contains("int main()"));
    }
}
