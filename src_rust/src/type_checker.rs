// MinRust Type Checker - Phase 5 Rust Implementation (완성)
// Type inference and validation with scope management

use crate::ast::*;
use std::collections::HashMap;

/// Type checking error
#[derive(Debug, Clone)]
pub struct TypeCheckError {
    pub message: String,
}

/// Type checker
pub struct TypeChecker {
    scopes: Vec<HashMap<String, RustType>>,
    functions: HashMap<String, FnDecl>,
    errors: Vec<TypeCheckError>,
}

impl TypeChecker {
    /// Create new type checker
    pub fn new() -> Self {
        TypeChecker {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            errors: Vec::new(),
        }
    }

    /// Check types in program
    pub fn check(&mut self, program: &Program) -> Result<(), Vec<TypeCheckError>> {
        // Stage 1: Collect all function definitions
        self.collect_functions(program);

        // Stage 2: Check each item
        for item in &program.items {
            self.check_item(item);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Collect function signatures
    fn collect_functions(&mut self, program: &Program) {
        for item in &program.items {
            if let Item::Function(fn_decl) = item {
                self.functions.insert(fn_decl.name.clone(), fn_decl.clone());
            }
        }
    }

    /// Check item type
    fn check_item(&mut self, item: &Item) {
        match item {
            Item::Function(fn_decl) => self.check_fn_decl(fn_decl),
            Item::Struct(struct_decl) => {
                // Register struct in current scope
                self.declare_symbol(
                    struct_decl.name.clone(),
                    RustType::Struct(struct_decl.name.clone()),
                );
            }
            Item::ImplBlock(impl_block) => {
                for method in &impl_block.methods {
                    self.check_fn_decl(method);
                }
            }
        }
    }

    /// Check function declaration
    fn check_fn_decl(&mut self, fn_decl: &FnDecl) {
        self.push_scope();

        // Declare parameters in function scope
        for param in &fn_decl.params {
            self.declare_symbol(param.name.clone(), param.ty.clone());
        }

        // Check body statements
        for stmt in &fn_decl.body {
            self.check_stmt(stmt);
        }

        self.pop_scope();
    }

    /// Check statement type
    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl {
                name,
                ty,
                init,
                mutable: _,
            } => {
                if let Some(init_expr) = init {
                    let expr_ty = self.check_expr(init_expr);
                    if let Some(declared_ty) = ty {
                        if !self.types_compatible(&expr_ty, declared_ty) {
                            self.error(format!(
                                "Type mismatch: expected {}, got {}",
                                declared_ty.to_string_repr(),
                                expr_ty.to_string_repr()
                            ));
                        }
                    }
                }
                let inferred_ty = if let Some(declared_ty) = ty {
                    declared_ty.clone()
                } else if let Some(init_expr) = init {
                    self.check_expr(init_expr)
                } else {
                    RustType::Primitive("()".to_string())
                };
                self.declare_symbol(name.clone(), inferred_ty);
            }
            Stmt::ConstDecl { name, ty, value } => {
                let expr_ty = self.check_expr(value);
                if let Some(declared_ty) = ty {
                    if !self.types_compatible(&expr_ty, declared_ty) {
                        self.error(format!(
                            "Type mismatch in const: expected {}, got {}",
                            declared_ty.to_string_repr(),
                            expr_ty.to_string_repr()
                        ));
                    }
                }
                let inferred_ty = if let Some(declared_ty) = ty {
                    declared_ty.clone()
                } else {
                    expr_ty
                };
                self.declare_symbol(name.clone(), inferred_ty);
            }
            Stmt::IfStmt {
                condition,
                then_body,
                else_body,
            } => {
                let cond_ty = self.check_expr(condition);
                if !matches!(cond_ty, RustType::Primitive(ref s) if s == "bool") {
                    self.error("If condition must be bool".to_string());
                }

                self.push_scope();
                for stmt in then_body {
                    self.check_stmt(stmt);
                }
                self.pop_scope();

                if let Some(else_stmts) = else_body {
                    self.push_scope();
                    for stmt in else_stmts {
                        self.check_stmt(stmt);
                    }
                    self.pop_scope();
                }
            }
            Stmt::ForStmt { var, iter, body } => {
                let iter_ty = self.check_expr(iter);

                self.push_scope();

                // Determine element type from iterator
                let elem_ty = match iter_ty {
                    RustType::Array { element, .. } => *element,
                    RustType::Vec(elem) => *elem,
                    _ => RustType::Primitive("i32".to_string()),
                };

                self.declare_symbol(var.clone(), elem_ty);

                for stmt in body {
                    self.check_stmt(stmt);
                }

                self.pop_scope();
            }
            Stmt::WhileStmt { condition, body } => {
                let cond_ty = self.check_expr(condition);
                if !matches!(cond_ty, RustType::Primitive(ref s) if s == "bool") {
                    self.error("While condition must be bool".to_string());
                }

                self.push_scope();
                for stmt in body {
                    self.check_stmt(stmt);
                }
                self.pop_scope();
            }
            Stmt::LoopStmt { body } => {
                self.push_scope();
                for stmt in body {
                    self.check_stmt(stmt);
                }
                self.pop_scope();
            }
            Stmt::MatchStmt { expr, arms } => {
                let _ = self.check_expr(expr);

                for (_, body) in arms {
                    self.push_scope();
                    for stmt in body {
                        self.check_stmt(stmt);
                    }
                    self.pop_scope();
                }
            }
            Stmt::ReturnStmt(expr) => {
                if let Some(ret_expr) = expr {
                    let _ = self.check_expr(ret_expr);
                }
            }
            Stmt::ExprStmt(expr) => {
                let _ = self.check_expr(expr);
            }
            Stmt::Block(stmts) => {
                self.push_scope();
                for stmt in stmts {
                    self.check_stmt(stmt);
                }
                self.pop_scope();
            }
        }
    }

    /// Check expression and return its type
    fn check_expr(&mut self, expr: &Expr) -> RustType {
        match expr {
            Expr::Literal(lit) => self.infer_literal_type(lit),
            Expr::Identifier(name) => self.lookup_symbol(name).unwrap_or(RustType::Primitive("unknown".to_string())),
            Expr::BinaryOp { left, op, right } => {
                let left_ty = self.check_expr(left);
                let right_ty = self.check_expr(right);

                // Type compatibility check
                if !self.types_compatible(&left_ty, &right_ty) {
                    self.error(format!(
                        "Type mismatch in binary op: {} and {}",
                        left_ty.to_string_repr(),
                        right_ty.to_string_repr()
                    ));
                }

                self.binary_op_result_type(op, &left_ty, &right_ty)
            }
            Expr::UnaryOp { op, operand } => {
                let op_ty = self.check_expr(operand);
                self.unary_op_result_type(op, &op_ty)
            }
            Expr::Call { func, args } => {
                // Look up function
                if let Some(fn_decl) = self.functions.get(func) {
                    // Check argument count
                    if args.len() != fn_decl.params.len() {
                        self.error(format!(
                            "Function {} expects {} args, got {}",
                            func,
                            fn_decl.params.len(),
                            args.len()
                        ));
                    }

                    // Check argument types
                    for (arg, param) in args.iter().zip(fn_decl.params.iter()) {
                        let arg_ty = self.check_expr(arg);
                        if !self.types_compatible(&arg_ty, &param.ty) {
                            self.error(format!(
                                "Argument type mismatch: expected {}, got {}",
                                param.ty.to_string_repr(),
                                arg_ty.to_string_repr()
                            ));
                        }
                    }

                    fn_decl
                        .return_type
                        .clone()
                        .unwrap_or(RustType::Primitive("()".to_string()))
                } else {
                    self.error(format!("Unknown function: {}", func));
                    RustType::Primitive("unknown".to_string())
                }
            }
            Expr::FieldAccess { object, field } => {
                let _ = self.check_expr(object);
                RustType::Primitive("unknown".to_string())
            }
            Expr::Index { object, index } => {
                let obj_ty = self.check_expr(object);
                let _ = self.check_expr(index);

                match obj_ty {
                    RustType::Array { element, .. } => *element,
                    RustType::Vec(elem) => *elem,
                    _ => RustType::Primitive("unknown".to_string()),
                }
            }
            Expr::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    RustType::Array {
                        element: Box::new(RustType::Primitive("unknown".to_string())),
                        size: Some(0),
                    }
                } else {
                    let elem_ty = self.check_expr(&elements[0]);
                    for elem in elements.iter().skip(1) {
                        let ty = self.check_expr(elem);
                        if !self.types_compatible(&elem_ty, &ty) {
                            self.error("Array element type mismatch".to_string());
                        }
                    }
                    RustType::Array {
                        element: Box::new(elem_ty),
                        size: Some(elements.len()),
                    }
                }
            }
            Expr::StructLiteral { name, fields: _ } => RustType::Struct(name.clone()),
            Expr::Cast { expr: _, ty } => ty.clone(),
            Expr::Range { start: _, end: _ } => RustType::Primitive("Range".to_string()),
            Expr::MethodCall { object: _, method: _, args: _ } => {
                RustType::Primitive("unknown".to_string())
            }
        }
    }

    /// Infer type of literal
    fn infer_literal_type(&self, lit: &Literal) -> RustType {
        match lit {
            Literal::Integer(_) => RustType::Primitive("i32".to_string()),
            Literal::Float(_) => RustType::Primitive("f64".to_string()),
            Literal::String(_) => RustType::Primitive("String".to_string()),
            Literal::Char(_) => RustType::Primitive("char".to_string()),
            Literal::Bool(_) => RustType::Primitive("bool".to_string()),
        }
    }

    /// Get result type of binary operation
    fn binary_op_result_type(&self, op: &str, lhs: &RustType, rhs: &RustType) -> RustType {
        match op {
            "+" | "-" | "*" | "/" | "%" => {
                // Arithmetic operators return same numeric type
                if lhs.is_numeric() {
                    lhs.clone()
                } else {
                    RustType::Primitive("unknown".to_string())
                }
            }
            "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                // Comparison operators return bool
                RustType::Primitive("bool".to_string())
            }
            "&&" | "||" => {
                // Logical operators return bool
                RustType::Primitive("bool".to_string())
            }
            "=" => lhs.clone(),
            _ => RustType::Primitive("unknown".to_string()),
        }
    }

    /// Get result type of unary operation
    fn unary_op_result_type(&self, op: &str, operand: &RustType) -> RustType {
        match op {
            "-" => {
                if operand.is_numeric() {
                    operand.clone()
                } else {
                    RustType::Primitive("unknown".to_string())
                }
            }
            "!" => RustType::Primitive("bool".to_string()),
            "&" | "&mut" => RustType::Reference {
                inner: Box::new(operand.clone()),
                mutable: op == "&mut",
            },
            "*" => {
                if let RustType::Reference { inner, .. } = operand {
                    (**inner).clone()
                } else {
                    RustType::Primitive("unknown".to_string())
                }
            }
            _ => RustType::Primitive("unknown".to_string()),
        }
    }

    /// Check if types are compatible
    pub fn types_compatible(&self, lhs: &RustType, rhs: &RustType) -> bool {
        lhs == rhs
    }

    /// Push new scope
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop scope
    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Declare symbol in current scope
    fn declare_symbol(&mut self, name: String, ty: RustType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    /// Lookup symbol in any scope
    fn lookup_symbol(&self, name: &str) -> Option<RustType> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn error(&mut self, message: String) {
        self.errors.push(TypeCheckError { message });
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checker_creation() {
        let checker = TypeChecker::new();
        assert_eq!(checker.scopes.len(), 1);
    }

    #[test]
    fn test_type_compatibility() {
        let checker = TypeChecker::new();
        let ty1 = RustType::Primitive("i32".to_string());
        let ty2 = RustType::Primitive("i32".to_string());
        assert!(checker.types_compatible(&ty1, &ty2));
    }

    #[test]
    fn test_literal_type_inference() {
        let checker = TypeChecker::new();
        assert!(matches!(
            checker.infer_literal_type(&Literal::Integer(42)),
            RustType::Primitive(ref s) if s == "i32"
        ));
    }

    #[test]
    fn test_binary_op_arithmetic() {
        let checker = TypeChecker::new();
        let int_type = RustType::Primitive("i32".to_string());
        let result = checker.binary_op_result_type("+", &int_type, &int_type);
        assert_eq!(result, int_type);
    }

    #[test]
    fn test_comparison_op_returns_bool() {
        let checker = TypeChecker::new();
        let int_type = RustType::Primitive("i32".to_string());
        let result = checker.binary_op_result_type("==", &int_type, &int_type);
        match result {
            RustType::Primitive(s) => assert_eq!(s, "bool"),
            _ => panic!("Expected bool type"),
        }
    }

    #[test]
    fn test_scope_management() {
        let mut checker = TypeChecker::new();
        checker.push_scope();
        assert_eq!(checker.scopes.len(), 2);
        checker.pop_scope();
        assert_eq!(checker.scopes.len(), 1);
    }

    #[test]
    fn test_symbol_declaration_and_lookup() {
        let mut checker = TypeChecker::new();
        let ty = RustType::Primitive("i32".to_string());
        checker.declare_symbol("x".to_string(), ty.clone());
        assert_eq!(checker.lookup_symbol("x"), Some(ty));
    }

    #[test]
    fn test_unary_negation() {
        let checker = TypeChecker::new();
        let int_type = RustType::Primitive("i32".to_string());
        let result = checker.unary_op_result_type("-", &int_type);
        assert_eq!(result, int_type);
    }

    #[test]
    fn test_unary_not() {
        let checker = TypeChecker::new();
        let bool_type = RustType::Primitive("bool".to_string());
        let result = checker.unary_op_result_type("!", &bool_type);
        match result {
            RustType::Primitive(s) => assert_eq!(s, "bool"),
            _ => panic!("Expected bool type"),
        }
    }

    #[test]
    fn test_reference_type() {
        let checker = TypeChecker::new();
        let int_type = RustType::Primitive("i32".to_string());
        let result = checker.unary_op_result_type("&", &int_type);
        match result {
            RustType::Reference { inner, mutable } => {
                assert_eq!(*inner, int_type);
                assert!(!mutable);
            }
            _ => panic!("Expected reference type"),
        }
    }
}
