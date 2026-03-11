# MinRust Parser - Syntax Analysis (Recursive Descent)
# 토큰 스트림을 AST로 변환

module MinRustParser

using ..MinRustTokenizer
using ..MinRustAST

export parse

# ============================================================
# Parser Structure
# ============================================================

mutable struct Parser
    tokens::Vector{Token}
    pos::Int
    errors::Vector{String}
end

function Parser(tokens::Vector{Token})
    return Parser(tokens, 1, String[])
end

# ============================================================
# Helper Functions
# ============================================================

function current(parser::Parser)::Token
    if parser.pos > length(parser.tokens)
        return parser.tokens[end]  # EOF
    end
    return parser.tokens[parser.pos]
end

function peek(parser::Parser, offset::Int = 0)::Token
    pos = parser.pos + offset
    if pos > length(parser.tokens)
        return parser.tokens[end]
    end
    return parser.tokens[pos]
end

function advance!(parser::Parser)::Token
    token = current(parser)
    if current(parser).type != EOF_TOKEN
        parser.pos += 1
    end
    return token
end

function match_type(parser::Parser, types...)::Bool
    return current(parser).type in types
end

function consume!(parser::Parser, type::TokenType, message::String = "")::Token
    if current(parser).type == type
        return advance!(parser)
    end
    error_msg = "Expected $type but got $(current(parser).type)"
    if message != ""
        error_msg *= ": $message"
    end
    push!(parser.errors, error_msg)
    return current(parser)
end

function synchronize!(parser::Parser)
    advance!(parser)
    while current(parser).type != EOF_TOKEN
        if current(parser).type in [SEMICOLON, KW_FN, KW_STRUCT, KW_IF, KW_FOR, KW_WHILE]
            return
        end
        advance!(parser)
    end
end

# ============================================================
# Main Parsing Functions
# ============================================================

function parse(tokens::Vector{Token})::Program
    parser = Parser(tokens)
    items = ASTNode[]

    while current(parser).type != EOF_TOKEN
        if current(parser).type == KW_FN
            push!(items, parse_fn_decl(parser))
        elseif current(parser).type == KW_STRUCT
            push!(items, parse_struct_decl(parser))
        elseif current(parser).type == KW_IMPL
            push!(items, parse_impl_block(parser))
        else
            advance!(parser)
        end
    end

    return Program(items)
end

# ============================================================
# Declaration Parsing
# ============================================================

function parse_fn_decl(parser::Parser)::FnDecl
    consume!(parser, KW_FN)
    name_token = consume!(parser, IDENTIFIER)
    name = name_token.value::String

    consume!(parser, LPAREN)
    params = parse_param_list(parser)
    consume!(parser, RPAREN)

    return_type = nothing
    if match_type(parser, ARROW)
        advance!(parser)
        return_type = parse_type(parser)
    end

    body = parse_block(parser)

    return FnDecl(name, params, return_type, body)
end

function parse_param_list(parser::Parser)::Vector{Param}
    params = Param[]

    if !match_type(parser, RPAREN)
        while true
            name_token = consume!(parser, IDENTIFIER)
            name = name_token.value::String
            consume!(parser, COLON)
            type_annotation = parse_type(parser)

            push!(params, Param(name, type_annotation))

            if !match_type(parser, COMMA)
                break
            end
            advance!(parser)
        end
    end

    return params
end

function parse_struct_decl(parser::Parser)::StructDecl
    consume!(parser, KW_STRUCT)
    name_token = consume!(parser, IDENTIFIER)
    name = name_token.value::String

    consume!(parser, LBRACE)
    fields = StructField[]

    while !match_type(parser, RBRACE) && current(parser).type != EOF_TOKEN
        field_name_token = consume!(parser, IDENTIFIER)
        field_name = field_name_token.value::String
        consume!(parser, COLON)
        field_type = parse_type(parser)

        push!(fields, StructField(field_name, field_type))

        if match_type(parser, COMMA)
            advance!(parser)
        end
    end

    consume!(parser, RBRACE)
    return StructDecl(name, fields)
end

function parse_impl_block(parser::Parser)::ImplBlock
    consume!(parser, KW_IMPL)
    type_name_token = consume!(parser, IDENTIFIER)
    for_type = type_name_token.value::String

    consume!(parser, LBRACE)
    methods = FnDecl[]

    while !match_type(parser, RBRACE) && current(parser).type != EOF_TOKEN
        if match_type(parser, KW_FN)
            push!(methods, parse_fn_decl(parser))
        else
            advance!(parser)
        end
    end

    consume!(parser, RBRACE)
    return ImplBlock(for_type, methods)
end

# ============================================================
# Type Parsing
# ============================================================

function parse_type(parser::Parser)::RustType
    # Reference type
    if match_type(parser, AMPERSAND)
        advance!(parser)
        is_mutable = match_type(parser, KW_MUT)
        if is_mutable
            advance!(parser)
        end
        inner = parse_type(parser)
        return ReferenceType(inner, is_mutable)
    end

    # Array type
    if match_type(parser, LBRACKET)
        advance!(parser)
        element_type = parse_type(parser)
        consume!(parser, RBRACKET)
        return ArrayType(element_type, nothing)
    end

    # Primitive or struct type
    name_token = consume!(parser, IDENTIFIER)
    type_name = name_token.value::String

    if type_name in ["i32", "i64", "u32", "u64", "f64", "bool", "char"]
        return PrimitiveType(type_name)
    elseif type_name == "String"
        return PrimitiveType("String")
    elseif type_name == "Vec"
        consume!(parser, LT)
        inner = parse_type(parser)
        consume!(parser, GT)
        return VecType(inner)
    else
        return StructType(type_name)
    end
end

# ============================================================
# Block Parsing
# ============================================================

function parse_block(parser::Parser)::Block
    consume!(parser, LBRACE)
    statements = ASTNode[]

    while !match_type(parser, RBRACE) && current(parser).type != EOF_TOKEN
        stmt = parse_statement(parser)
        if stmt !== nothing
            push!(statements, stmt)
        end
    end

    consume!(parser, RBRACE)
    return Block(statements)
end

# ============================================================
# Statement Parsing
# ============================================================

function parse_statement(parser::Parser)::Union{Stmt, Nothing}
    if match_type(parser, KW_LET)
        return parse_var_decl(parser)
    elseif match_type(parser, KW_CONST)
        return parse_const_decl(parser)
    elseif match_type(parser, KW_IF)
        return parse_if_stmt(parser)
    elseif match_type(parser, KW_FOR)
        return parse_for_stmt(parser)
    elseif match_type(parser, KW_WHILE)
        return parse_while_stmt(parser)
    elseif match_type(parser, KW_LOOP)
        return parse_loop_stmt(parser)
    elseif match_type(parser, KW_MATCH)
        return parse_match_stmt(parser)
    elseif match_type(parser, KW_RETURN)
        return parse_return_stmt(parser)
    elseif match_type(parser, KW_BREAK)
        return parse_break_stmt(parser)
    elseif match_type(parser, KW_CONTINUE)
        return parse_continue_stmt(parser)
    else
        return parse_expr_stmt(parser)
    end
end

function parse_var_decl(parser::Parser)::VarDecl
    consume!(parser, KW_LET)

    is_mutable = false
    if match_type(parser, KW_MUT)
        is_mutable = true
        advance!(parser)
    end

    name_token = consume!(parser, IDENTIFIER)
    name = name_token.value::String

    type_annotation = nothing
    if match_type(parser, COLON)
        advance!(parser)
        type_annotation = parse_type(parser)
    end

    init = nothing
    if match_type(parser, ASSIGN)
        advance!(parser)
        init = parse_expression(parser)
    end

    if match_type(parser, SEMICOLON)
        advance!(parser)
    end

    return VarDecl(name, type_annotation, is_mutable, init)
end

function parse_const_decl(parser::Parser)::VarDecl
    consume!(parser, KW_CONST)
    name_token = consume!(parser, IDENTIFIER)
    name = name_token.value::String

    consume!(parser, COLON)
    type_annotation = parse_type(parser)

    consume!(parser, ASSIGN)
    init = parse_expression(parser)

    if match_type(parser, SEMICOLON)
        advance!(parser)
    end

    return VarDecl(name, type_annotation, false, init)
end

function parse_if_stmt(parser::Parser)::IfStmt
    consume!(parser, KW_IF)
    condition = parse_expression(parser)
    then_block = parse_block(parser)

    else_block = nothing
    if match_type(parser, KW_ELSE)
        advance!(parser)
        if match_type(parser, KW_IF)
            # else if - convert to nested if
            nested_if = parse_if_stmt(parser)
            else_block = Block([nested_if])
        else
            else_block = parse_block(parser)
        end
    end

    return IfStmt(condition, then_block, else_block)
end

function parse_for_stmt(parser::Parser)::ForStmt
    consume!(parser, KW_FOR)
    var_token = consume!(parser, IDENTIFIER)
    var = var_token.value::String

    consume!(parser, KW_IN)
    iter_expr = parse_expression(parser)

    body = parse_block(parser)

    return ForStmt(var, iter_expr, body)
end

function parse_while_stmt(parser::Parser)::WhileStmt
    consume!(parser, KW_WHILE)
    condition = parse_expression(parser)
    body = parse_block(parser)

    return WhileStmt(condition, body)
end

function parse_loop_stmt(parser::Parser)::LoopStmt
    consume!(parser, KW_LOOP)
    body = parse_block(parser)

    return LoopStmt(body)
end

function parse_match_stmt(parser::Parser)::MatchStmt
    consume!(parser, KW_MATCH)
    expr = parse_expression(parser)

    consume!(parser, LBRACE)
    arms = MatchArm[]

    while !match_type(parser, RBRACE) && current(parser).type != EOF_TOKEN
        pattern_token = consume!(parser, IDENTIFIER)
        pattern = pattern_token.value::String

        consume!(parser, FAT_ARROW)
        arm_expr = parse_expression(parser)

        push!(arms, MatchArm(pattern, arm_expr))

        if match_type(parser, COMMA)
            advance!(parser)
        end
    end

    consume!(parser, RBRACE)
    return MatchStmt(expr, arms)
end

function parse_return_stmt(parser::Parser)::ReturnStmt
    consume!(parser, KW_RETURN)

    value = nothing
    if !match_type(parser, SEMICOLON) && !match_type(parser, RBRACE)
        value = parse_expression(parser)
    end

    if match_type(parser, SEMICOLON)
        advance!(parser)
    end

    return ReturnStmt(value)
end

function parse_break_stmt(parser::Parser)::BreakStmt
    consume!(parser, KW_BREAK)
    if match_type(parser, SEMICOLON)
        advance!(parser)
    end
    return BreakStmt(nothing)
end

function parse_continue_stmt(parser::Parser)::ContinueStmt
    consume!(parser, KW_CONTINUE)
    if match_type(parser, SEMICOLON)
        advance!(parser)
    end
    return ContinueStmt(nothing)
end

function parse_expr_stmt(parser::Parser)::ExprStmt
    expr = parse_expression(parser)

    if match_type(parser, SEMICOLON)
        advance!(parser)
    end

    return ExprStmt(expr)
end

# ============================================================
# Expression Parsing (Recursive Descent with Operator Precedence)
# ============================================================

function parse_expression(parser::Parser)::Expr
    return parse_assignment(parser)
end

function parse_assignment(parser::Parser)::Expr
    expr = parse_logical_or(parser)

    if match_type(parser, ASSIGN)
        advance!(parser)
        right = parse_assignment(parser)
        return BinaryOp(expr, "=", right)
    end

    return expr
end

function parse_logical_or(parser::Parser)::Expr
    expr = parse_logical_and(parser)

    while match_type(parser, OR)
        op = advance!(parser)
        right = parse_logical_and(parser)
        expr = BinaryOp(expr, "||", right)
    end

    return expr
end

function parse_logical_and(parser::Parser)::Expr
    expr = parse_equality(parser)

    while match_type(parser, AND)
        op = advance!(parser)
        right = parse_equality(parser)
        expr = BinaryOp(expr, "&&", right)
    end

    return expr
end

function parse_equality(parser::Parser)::Expr
    expr = parse_comparison(parser)

    while match_type(parser, EQ, NE)
        op_token = advance!(parser)
        op_str = (op_token.type == EQ) ? "==" : "!="
        right = parse_comparison(parser)
        expr = BinaryOp(expr, op_str, right)
    end

    return expr
end

function parse_comparison(parser::Parser)::Expr
    expr = parse_additive(parser)

    while match_type(parser, LT, GT, LE, GE)
        op_token = advance!(parser)
        op_str = string(Char(convert(Int, op_token.type)))  # Convert to operator
        if op_token.type == LT
            op_str = "<"
        elseif op_token.type == GT
            op_str = ">"
        elseif op_token.type == LE
            op_str = "<="
        elseif op_token.type == GE
            op_str = ">="
        end
        right = parse_additive(parser)
        expr = BinaryOp(expr, op_str, right)
    end

    return expr
end

function parse_additive(parser::Parser)::Expr
    expr = parse_multiplicative(parser)

    while match_type(parser, PLUS, MINUS)
        op_token = advance!(parser)
        op_str = (op_token.type == PLUS) ? "+" : "-"
        right = parse_multiplicative(parser)
        expr = BinaryOp(expr, op_str, right)
    end

    return expr
end

function parse_multiplicative(parser::Parser)::Expr
    expr = parse_unary(parser)

    while match_type(parser, STAR, SLASH, PERCENT)
        op_token = advance!(parser)
        op_str = (op_token.type == STAR) ? "*" : (op_token.type == SLASH) ? "/" : "%"
        right = parse_unary(parser)
        expr = BinaryOp(expr, op_str, right)
    end

    return expr
end

function parse_unary(parser::Parser)::Expr
    if match_type(parser, MINUS, NOT, AMPERSAND, STAR)
        op_token = advance!(parser)
        op_str = (op_token.type == MINUS) ? "-" : (op_token.type == NOT) ? "!" :
                 (op_token.type == AMPERSAND) ? "&" : "*"
        expr = parse_unary(parser)
        return UnaryOp(op_str, expr)
    end

    return parse_postfix(parser)
end

function parse_postfix(parser::Parser)::Expr
    expr = parse_primary(parser)

    while true
        if match_type(parser, LPAREN)
            # Function call
            advance!(parser)
            args = Expr[]

            if !match_type(parser, RPAREN)
                while true
                    push!(args, parse_expression(parser))
                    if !match_type(parser, COMMA)
                        break
                    end
                    advance!(parser)
                end
            end

            consume!(parser, RPAREN)
            if isa(expr, Identifier)
                expr = Call(expr.name, args)
            else
                expr = Call(expr, args)
            end

        elseif match_type(parser, LBRACKET)
            # Array index
            advance!(parser)
            index = parse_expression(parser)
            consume!(parser, RBRACKET)
            expr = Index(expr, index)

        elseif match_type(parser, DOT)
            # Field access or method call
            advance!(parser)
            name_token = consume!(parser, IDENTIFIER)
            name = name_token.value::String

            if match_type(parser, LPAREN)
                # Method call
                advance!(parser)
                args = Expr[]

                if !match_type(parser, RPAREN)
                    while true
                        push!(args, parse_expression(parser))
                        if !match_type(parser, COMMA)
                            break
                        end
                        advance!(parser)
                    end
                end

                consume!(parser, RPAREN)
                expr = MethodCall(expr, name, args)
            else
                # Field access
                expr = FieldAccess(expr, name)
            end

        elseif match_type(parser, KW_AS)
            # Cast
            advance!(parser)
            target_type = parse_type(parser)
            expr = Cast(expr, target_type)

        else
            break
        end
    end

    return expr
end

function parse_primary(parser::Parser)::Expr
    # Integer literal
    if match_type(parser, INT_LITERAL)
        token = advance!(parser)
        return Literal(token.value::Int)
    end

    # Float literal
    if match_type(parser, FLOAT_LITERAL)
        token = advance!(parser)
        return Literal(token.value::Float)
    end

    # String literal
    if match_type(parser, STRING_LITERAL)
        token = advance!(parser)
        return Literal(token.value::String)
    end

    # Char literal
    if match_type(parser, CHAR_LITERAL)
        token = advance!(parser)
        return Literal(token.value::Char)
    end

    # Boolean literals
    if match_type(parser, KW_TRUE)
        advance!(parser)
        return Literal(true)
    end

    if match_type(parser, KW_FALSE)
        advance!(parser)
        return Literal(false)
    end

    # Identifier
    if match_type(parser, IDENTIFIER)
        token = advance!(parser)
        return Identifier(token.value::String)
    end

    # Range
    if match_type(parser, DOTDOT)
        advance!(parser)
        end_expr = parse_primary(parser)
        return Range(nothing, end_expr, false)
    end

    # Grouped expression
    if match_type(parser, LPAREN)
        advance!(parser)
        expr = parse_expression(parser)
        consume!(parser, RPAREN)
        return expr
    end

    # Array literal
    if match_type(parser, LBRACKET)
        advance!(parser)
        elements = Expr[]

        if !match_type(parser, RBRACKET)
            while true
                push!(elements, parse_expression(parser))
                if !match_type(parser, COMMA)
                    break
                end
                advance!(parser)
            end
        end

        consume!(parser, RBRACKET)
        return ArrayLiteral(elements)
    end

    error("Unexpected token: $(current(parser).type)")
end

end  # module MinRustParser
