// MinRust AST - Phase 5 Rust Implementation
// Abstract Syntax Tree structures

use crate::tokenizer::Token;

/// Top-level program structure
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

/// Top-level items
#[derive(Debug, Clone)]
pub enum Item {
    Function(FnDecl),
    Struct(StructDecl),
    ImplBlock(ImplBlock),
}

/// Function declaration
#[derive(Debug, Clone)]
pub struct FnDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<RustType>,
    pub body: Vec<Stmt>,
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: RustType,
}

/// Struct declaration
#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<StructField>,
}

/// Struct field
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: RustType,
}

/// Implementation block
#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub target: String,
    pub methods: Vec<FnDecl>,
}

/// Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        name: String,
        ty: Option<RustType>,
        init: Option<Expr>,
        mutable: bool,
    },
    ConstDecl {
        name: String,
        ty: Option<RustType>,
        value: Expr,
    },
    IfStmt {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    ForStmt {
        var: String,
        iter: Expr,
        body: Vec<Stmt>,
    },
    WhileStmt {
        condition: Expr,
        body: Vec<Stmt>,
    },
    LoopStmt {
        body: Vec<Stmt>,
    },
    MatchStmt {
        expr: Expr,
        arms: Vec<(String, Vec<Stmt>)>,
    },
    ReturnStmt(Option<Expr>),
    ExprStmt(Expr),
    Block(Vec<Stmt>),
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    UnaryOp {
        op: String,
        operand: Box<Expr>,
    },
    Call {
        func: String,
        args: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    FieldAccess {
        object: Box<Expr>,
        field: String,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    ArrayLiteral(Vec<Expr>),
    StructLiteral {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    Cast {
        expr: Box<Expr>,
        ty: RustType,
    },
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },
}

/// Literal values
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

/// Rust types
#[derive(Debug, Clone, PartialEq)]
pub enum RustType {
    Primitive(String), // i32, f64, bool, char, String, etc
    Reference {
        inner: Box<RustType>,
        mutable: bool,
    },
    Array {
        element: Box<RustType>,
        size: Option<usize>,
    },
    Vec(Box<RustType>),
    Struct(String),
    Function {
        params: Vec<RustType>,
        return_ty: Box<RustType>,
    },
}

impl RustType {
    /// Check if type is primitive
    pub fn is_primitive(&self) -> bool {
        matches!(self, RustType::Primitive(_))
    }

    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        match self {
            RustType::Primitive(name) => {
                matches!(name.as_str(), "i32" | "i64" | "f64" | "u32" | "u64")
            }
            _ => false,
        }
    }

    /// Get string representation
    pub fn to_string_repr(&self) -> String {
        match self {
            RustType::Primitive(name) => name.clone(),
            RustType::Reference { inner, mutable } => {
                let mut_str = if *mutable { "mut " } else { "" };
                format!("&{}{}", mut_str, inner.to_string_repr())
            }
            RustType::Array { element, size } => {
                if let Some(s) = size {
                    format!("[{}; {}]", element.to_string_repr(), s)
                } else {
                    format!("[{}]", element.to_string_repr())
                }
            }
            RustType::Vec(inner) => format!("Vec<{}>", inner.to_string_repr()),
            RustType::Struct(name) => name.clone(),
            RustType::Function { params, return_ty } => {
                let param_str = params
                    .iter()
                    .map(|p| p.to_string_repr())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", param_str, return_ty.to_string_repr())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_primitive() {
        let ty = RustType::Primitive("i32".to_string());
        assert!(ty.is_primitive());
        assert!(ty.is_numeric());
    }

    #[test]
    fn test_type_reference() {
        let inner = Box::new(RustType::Primitive("i32".to_string()));
        let ty = RustType::Reference {
            inner,
            mutable: false,
        };
        assert!(!ty.is_primitive());
    }
}
