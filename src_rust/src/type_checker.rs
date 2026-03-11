// MinRust Type Checker - Phase 5 Rust Implementation (Skeleton)
// Type inference and validation - Full implementation in Phase 5.3

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
    functions: HashMap<String, RustType>,
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
        // TODO: Implement full type checking
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
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
}
