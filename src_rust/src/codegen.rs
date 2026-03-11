// MinRust Code Generator - Phase 5 Rust Implementation (완성)
// Generates C code from typed AST

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
        // Add include directives
        self.output.push_str("#include <stdio.h>\n");
        self.output.push_str("#include <stdlib.h>\n");
        self.output.push_str("#include <string.h>\n");
        self.output.push_str("#include \"minrust_stdlib.h\"\n\n");

        // Forward declarations for functions
        for item in &program.items {
            if let Item::Function(fn_decl) = item {
                self.gen_fn_prototype(fn_decl);
            }
        }

        self.output.push('\n');

        // Function implementations
        for item in &program.items {
            match item {
                Item::Function(fn_decl) => self.gen_fn_decl(fn_decl),
                Item::Struct(struct_decl) => self.gen_struct_decl(struct_decl),
                _ => {}
            }
        }

        self.output.clone()
    }

    /// Generate function prototype
    fn gen_fn_prototype(&mut self, fn_decl: &FnDecl) {
        let return_type = fn_decl
            .return_type
            .as_ref()
            .map(|t| self.rust_type_to_c(t))
            .unwrap_or_else(|| "void".to_string());

        self.output.push_str(&format!("{} {}(", return_type, fn_decl.name));

        for (i, param) in fn_decl.params.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            let param_type = self.rust_type_to_c(&param.ty);
            self.output.push_str(&format!("{} {}", param_type, param.name));
        }

        self.output.push_str(");\n");
    }

    /// Generate function definition
    fn gen_fn_decl(&mut self, fn_decl: &FnDecl) {
        let return_type = fn_decl
            .return_type
            .as_ref()
            .map(|t| self.rust_type_to_c(t))
            .unwrap_or_else(|| "void".to_string());

        self.output.push_str(&format!("{} {}(", return_type, fn_decl.name));

        for (i, param) in fn_decl.params.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            let param_type = self.rust_type_to_c(&param.ty);
            self.output.push_str(&format!("{} {}", param_type, param.name));
        }

        self.output.push_str(") {\n");

        self.indent();

        for stmt in &fn_decl.body {
            self.gen_stmt(stmt);
        }

        self.unindent();

        self.output.push_str("}\n\n");
    }

    /// Generate struct definition
    fn gen_struct_decl(&mut self, struct_decl: &StructDecl) {
        self.output.push_str(&format!("typedef struct {{\n", ));

        self.indent();
        for field in &struct_decl.fields {
            let field_type = self.rust_type_to_c(&field.ty);
            self.output.push_str(&format!("{}{}  {};\n", self.indent_str(), field_type, field.name));
        }
        self.unindent();

        self.output.push_str(&format!("}} {};\n\n", struct_decl.name));
    }

    /// Generate statement
    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl {
                name,
                ty,
                init,
                mutable: _,
            } => {
                let var_type = ty
                    .as_ref()
                    .map(|t| self.rust_type_to_c(t))
                    .unwrap_or_else(|| "int".to_string());

                self.output.push_str(&format!("{}{}", self.indent_str(), var_type));
                self.output.push(' ');
                self.output.push_str(name);

                if let Some(init_expr) = init {
                    self.output.push_str(" = ");
                    self.output.push_str(&self.gen_expr(init_expr));
                }

                self.output.push_str(";\n");
            }
            Stmt::ConstDecl { name, ty, value } => {
                let var_type = ty
                    .as_ref()
                    .map(|t| self.rust_type_to_c(t))
                    .unwrap_or_else(|| "int".to_string());

                self.output.push_str(&format!(
                    "{}const {} {} = {};\n",
                    self.indent_str(),
                    var_type,
                    name,
                    self.gen_expr(value)
                ));
            }
            Stmt::IfStmt {
                condition,
                then_body,
                else_body,
            } => {
                self.output.push_str(&format!(
                    "{}if ({}) {{\n",
                    self.indent_str(),
                    self.gen_expr(condition)
                ));

                self.indent();
                for stmt in then_body {
                    self.gen_stmt(stmt);
                }
                self.unindent();

                if let Some(else_stmts) = else_body {
                    self.output.push_str(&format!("{}}} else {{\n", self.indent_str()));
                    self.indent();
                    for stmt in else_stmts {
                        self.gen_stmt(stmt);
                    }
                    self.unindent();
                }

                self.output.push_str(&format!("{}}}\n", self.indent_str()));
            }
            Stmt::ForStmt { var, iter, body } => {
                self.output.push_str(&format!(
                    "{}for (int {} in {}) {{\n",
                    self.indent_str(),
                    var,
                    self.gen_expr(iter)
                ));

                self.indent();
                for stmt in body {
                    self.gen_stmt(stmt);
                }
                self.unindent();

                self.output.push_str(&format!("{}}}\n", self.indent_str()));
            }
            Stmt::WhileStmt { condition, body } => {
                self.output.push_str(&format!(
                    "{}while ({}) {{\n",
                    self.indent_str(),
                    self.gen_expr(condition)
                ));

                self.indent();
                for stmt in body {
                    self.gen_stmt(stmt);
                }
                self.unindent();

                self.output.push_str(&format!("{}}}\n", self.indent_str()));
            }
            Stmt::LoopStmt { body } => {
                self.output.push_str(&format!("{}while (1) {{\n", self.indent_str()));

                self.indent();
                for stmt in body {
                    self.gen_stmt(stmt);
                }
                self.unindent();

                self.output.push_str(&format!("{}}}\n", self.indent_str()));
            }
            Stmt::ReturnStmt(expr) => {
                self.output.push_str(&format!("{}return", self.indent_str()));

                if let Some(ret_expr) = expr {
                    self.output.push(' ');
                    self.output.push_str(&self.gen_expr(ret_expr));
                }

                self.output.push_str(";\n");
            }
            Stmt::ExprStmt(expr) => {
                self.output.push_str(&format!("{}{};\n", self.indent_str(), self.gen_expr(expr)));
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.gen_stmt(stmt);
                }
            }
            Stmt::MatchStmt { expr: _, arms: _ } => {
                // Simplified: match not fully supported
            }
        }
    }

    /// Generate expression
    fn gen_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Integer(n) => format!("{}", n),
                Literal::Float(f) => format!("{}", f),
                Literal::String(s) => format!("\"{}\"", s),
                Literal::Char(c) => format!("'{}'", c),
                Literal::Bool(b) => format!("{}", if *b { "1" } else { "0" }),
            },
            Expr::Identifier(name) => name.clone(),
            Expr::BinaryOp { left, op, right } => {
                let left_code = self.gen_expr(left);
                let right_code = self.gen_expr(right);
                let c_op = self.operator_to_c(op);
                format!("({} {} {})", left_code, c_op, right_code)
            }
            Expr::UnaryOp { op, operand } => {
                let operand_code = self.gen_expr(operand);
                match op.as_str() {
                    "&" => format!("(&{})", operand_code),
                    "&mut" => format!("(&{})", operand_code),
                    "*" => format!("(*{})", operand_code),
                    "-" => format!("(-{})", operand_code),
                    "!" => format!("(!{})", operand_code),
                    _ => operand_code,
                }
            }
            Expr::Call { func, args } => {
                let args_code: Vec<String> = args.iter().map(|arg| self.gen_expr(arg)).collect();
                format!("{}({})", func, args_code.join(", "))
            }
            Expr::Index { object, index } => {
                let obj = self.gen_expr(object);
                let idx = self.gen_expr(index);
                format!("{}[{}]", obj, idx)
            }
            Expr::FieldAccess { object, field } => {
                let obj = self.gen_expr(object);
                format!("{}.{}", obj, field)
            }
            Expr::ArrayLiteral(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| self.gen_expr(e)).collect();
                format!("{{{}}}", elems.join(", "))
            }
            Expr::StructLiteral { name, fields } => {
                let mut code = format!("({{{}", name);
                for (field_name, field_expr) in fields {
                    code.push_str(&format!(" .{} = {}", field_name, self.gen_expr(field_expr)));
                }
                code.push_str("})");
                code
            }
            Expr::Cast { expr, ty } => {
                let expr_code = self.gen_expr(expr);
                let type_code = self.rust_type_to_c(ty);
                format!("(({}){})", type_code, expr_code)
            }
            Expr::Range { start: _, end: _ } => "range".to_string(), // Simplified
            Expr::MethodCall { object, method, args } => {
                let obj = self.gen_expr(object);
                let args_code: Vec<String> = args.iter().map(|arg| self.gen_expr(arg)).collect();
                format!("{}.{}({})", obj, method, args_code.join(", "))
            }
        }
    }

    /// Convert Rust operator to C operator
    fn operator_to_c(&self, op: &str) -> String {
        match op {
            "+" | "-" | "*" | "/" | "%" => op.to_string(),
            "==" | "!=" | "<" | "<=" | ">" | ">=" => op.to_string(),
            "&&" | "||" => op.to_string(),
            "=" => "=".to_string(),
            _ => op.to_string(),
        }
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
                "()" => "void".to_string(),
                "Range" => "int".to_string(), // Simplified
                _ => "int".to_string(),
            },
            RustType::Reference { inner, mutable: _ } => {
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
            RustType::Struct(name) => name.clone(),
            RustType::Function { .. } => "void*".to_string(), // Simplified
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
        assert!(code.contains("#include"));
    }

    #[test]
    fn test_operator_to_c() {
        let gen = CodeGenerator::new();
        assert_eq!(gen.operator_to_c("+"), "+");
        assert_eq!(gen.operator_to_c("=="), "==");
        assert_eq!(gen.operator_to_c("&&"), "&&");
    }

    #[test]
    fn test_gen_expr_literal() {
        let gen = CodeGenerator::new();
        assert_eq!(gen.gen_expr(&Expr::Literal(Literal::Integer(42))), "42");
    }

    #[test]
    fn test_gen_expr_identifier() {
        let gen = CodeGenerator::new();
        assert_eq!(gen.gen_expr(&Expr::Identifier("x".to_string())), "x");
    }

    #[test]
    fn test_indent_string() {
        let gen = CodeGenerator::new();
        assert_eq!(gen.indent_str(), "");

        let mut gen = CodeGenerator::new();
        gen.indent();
        assert_eq!(gen.indent_str(), "    ");

        gen.indent();
        assert_eq!(gen.indent_str(), "        ");
    }
}
