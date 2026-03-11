# MinRust AST - Abstract Syntax Tree Definitions

module MinRustAST

export ASTNode, Program, FnDecl, VarDecl, StructDecl, ImplBlock,
       Block, Param, StructField,
       Expr, Literal, Identifier, BinaryOp, UnaryOp, Call, MethodCall,
       FieldAccess, Index, ArrayLiteral, VecMacro, StructLiteral, Cast, Range,
       Stmt, IfStmt, ForStmt, WhileStmt, LoopStmt, MatchStmt, ReturnStmt, ExprStmt,
       RustType, PrimitiveType, ReferenceType, ArrayType, StructType

# ============================================================
# Base Types
# ============================================================

abstract type ASTNode end
abstract type Expr <: ASTNode end
abstract type Stmt <: ASTNode end
abstract type RustType end

# ============================================================
# Top-level Program
# ============================================================

struct Program <: ASTNode
    items::Vector{ASTNode}
end

# ============================================================
# Type Definitions
# ============================================================

struct PrimitiveType <: RustType
    name::String  # "i32", "i64", "f64", "bool", "String", etc.
end

struct ReferenceType <: RustType
    inner::RustType
    is_mutable::Bool
end

struct ArrayType <: RustType
    element_type::RustType
    size::Union{Int, Nothing}  # Nothing for slices
end

struct StructType <: RustType
    name::String
end

struct VecType <: RustType
    element_type::RustType
end

struct FunctionType <: RustType
    param_types::Vector{RustType}
    return_type::RustType
end

# ============================================================
# Function and Struct Declarations
# ============================================================

struct Param
    name::String
    type_annotation::RustType
end

struct FnDecl <: ASTNode
    name::String
    params::Vector{Param}
    return_type::Union{RustType, Nothing}
    body::Block
end

struct StructField
    name::String
    type_annotation::RustType
end

struct StructDecl <: ASTNode
    name::String
    fields::Vector{StructField}
end

struct ImplBlock <: ASTNode
    for_type::String
    methods::Vector{FnDecl}
end

# ============================================================
# Statements
# ============================================================

struct Block <: ASTNode
    statements::Vector{ASTNode}
end

struct VarDecl <: Stmt
    name::String
    type_annotation::Union{RustType, Nothing}
    is_mutable::Bool
    init::Union{Expr, Nothing}
end

struct ExprStmt <: Stmt
    expr::Expr
end

struct IfStmt <: Stmt
    condition::Expr
    then_block::Block
    else_block::Union{Block, Nothing}
end

struct ForStmt <: Stmt
    var::String
    iter_expr::Expr
    body::Block
end

struct WhileStmt <: Stmt
    condition::Expr
    body::Block
end

struct LoopStmt <: Stmt
    body::Block
end

struct MatchStmt <: Stmt
    expr::Expr
    arms::Vector{MatchArm}
end

struct MatchArm
    pattern::String  # simplified: just pattern string
    expr::Expr
end

struct ReturnStmt <: Stmt
    value::Union{Expr, Nothing}
end

struct BreakStmt <: Stmt
    label::Union{String, Nothing}
end

struct ContinueStmt <: Stmt
    label::Union{String, Nothing}
end

# ============================================================
# Expressions
# ============================================================

struct Literal <: Expr
    value::Union{Int, Float, String, Char, Bool}
end

struct Identifier <: Expr
    name::String
end

struct BinaryOp <: Expr
    left::Expr
    op::String  # "+", "-", "*", "/", "==", "!=", "<", ">", "<=", ">=", "&&", "||", etc.
    right::Expr
end

struct UnaryOp <: Expr
    op::String  # "-", "!", "&", "*", "~"
    operand::Expr
end

struct Call <: Expr
    func::Union{String, Expr}  # function name or expression
    args::Vector{Expr}
end

struct MethodCall <: Expr
    object::Expr
    method::String
    args::Vector{Expr}
end

struct FieldAccess <: Expr
    object::Expr
    field::String
end

struct Index <: Expr
    array::Expr
    index::Expr
end

struct ArrayLiteral <: Expr
    elements::Vector{Expr}
end

struct VecMacro <: Expr
    elements::Vector{Expr}
end

struct StructLiteral <: Expr
    struct_name::String
    fields::Vector{Pair{String, Expr}}
end

struct Cast <: Expr
    expr::Expr
    target_type::RustType
end

struct Range <: Expr
    start::Union{Expr, Nothing}
    end::Union{Expr, Nothing}
    inclusive::Bool
end

struct IfExpr <: Expr
    condition::Expr
    then_block::Block
    else_block::Union{Block, Nothing}
end

struct MatchExpr <: Expr
    expr::Expr
    arms::Vector{MatchArm}
end

struct Closure <: Expr
    params::Vector{String}
    body::Block
end

struct StringFormatExpr <: Expr
    format_str::String
    args::Vector{Expr}
end

end  # module MinRustAST
