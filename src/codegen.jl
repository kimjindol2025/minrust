# MinRust Code Generator - C Code Generation
# AST → C 코드로 변환

module MinRustCodeGen

using ..MinRustAST

export generate_code

# ============================================================
# Code Generator Structure
# ============================================================

mutable struct CodeGenerator
    output::IOBuffer
    var_counter::Int
    label_counter::Int
    indent_level::Int
    current_function::Union{String, Nothing}
end

function CodeGenerator()
    return CodeGenerator(IOBuffer(), 0, 0, 0, nothing)
end

# ============================================================
# Helper Functions
# ============================================================

function indent(gen::CodeGenerator)::String
    return repeat("  ", gen.indent_level)
end

function gen_temp_var(gen::CodeGenerator)::String
    gen.var_counter += 1
    return "__tmp_$(gen.var_counter)"
end

function gen_label(gen::CodeGenerator, prefix="label")::String
    gen.label_counter += 1
    return prefix * "_$(gen.label_counter)"
end

function println_code(gen::CodeGenerator, line::String)
    println(gen.output, indent(gen) * line)
end

function type_to_c(rust_type::RustType)::String
    if isa(rust_type, PrimitiveType)
        if rust_type.name == "i32"
            return "int"
        elseif rust_type.name == "i64"
            return "long long"
        elseif rust_type.name == "u32"
            return "unsigned int"
        elseif rust_type.name == "u64"
            return "unsigned long long"
        elseif rust_type.name == "f64"
            return "double"
        elseif rust_type.name == "bool"
            return "int"
        elseif rust_type.name == "char"
            return "char"
        elseif rust_type.name == "String"
            return "char*"
        else
            return "void"
        end
    elseif isa(rust_type, ReferenceType)
        inner_type = type_to_c(rust_type.inner)
        return inner_type * "*"
    elseif isa(rust_type, ArrayType)
        element_type = type_to_c(rust_type.element_type)
        if rust_type.size !== nothing
            return element_type * "[$(rust_type.size)]"
        else
            return element_type * "*"
        end
    elseif isa(rust_type, VecType)
        # Vector는 구조체로 표현 (간단화)
        return "vec_t"
    else
        return "void"
    end
end

# ============================================================
# Main Code Generation
# ============================================================

function generate_code(program::Program)::String
    gen = CodeGenerator()

    # 헤더
    println_code(gen, "#include <stdio.h>")
    println_code(gen, "#include <stdlib.h>")
    println_code(gen, "#include <string.h>")
    println_code(gen, "")

    # 전방 선언 (function prototypes)
    for item in program.items
        if isa(item, FnDecl)
            gen_fn_prototype(gen, item)
        end
    end

    if !isempty(program.items)
        println_code(gen, "")
    end

    # 코드 생성
    for item in program.items
        if isa(item, FnDecl)
            gen_fn_decl(gen, item)
        elseif isa(item, StructDecl)
            gen_struct_decl(gen, item)
        elseif isa(item, ImplBlock)
            gen_impl_block(gen, item)
        end
    end

    return String(take!(gen.output))
end

# ============================================================
# Function Code Generation
# ============================================================

function gen_fn_prototype(gen::CodeGenerator, fn::FnDecl)
    return_type_str = fn.return_type !== nothing ? type_to_c(fn.return_type) : "void"
    params = String[]

    for param in fn.params
        param_type = type_to_c(param.type_annotation)
        push!(params, param_type * " " * param.name)
    end

    param_str = join(params, ", ")
    if isempty(params)
        param_str = "void"
    end

    println_code(gen, "$return_type_str $(fn.name)($param_str);")
end

function gen_fn_decl(gen::CodeGenerator, fn::FnDecl)
    gen.current_function = fn.name
    return_type_str = fn.return_type !== nothing ? type_to_c(fn.return_type) : "void"
    params = String[]

    for param in fn.params
        param_type = type_to_c(param.type_annotation)
        push!(params, param_type * " " * param.name)
    end

    param_str = join(params, ", ")
    if isempty(params)
        param_str = "void"
    end

    println_code(gen, "$return_type_str $(fn.name)($param_str) {")
    gen.indent_level += 1

    gen_block(gen, fn.body)

    # 기본 반환값
    if return_type_str == "void"
        println_code(gen, "return;")
    end

    gen.indent_level -= 1
    println_code(gen, "}")
    println_code(gen, "")

    gen.current_function = nothing
end

function gen_struct_decl(gen::CodeGenerator, struct_decl::StructDecl)
    println_code(gen, "struct $(struct_decl.name) {")
    gen.indent_level += 1

    for field in struct_decl.fields
        field_type = type_to_c(field.type_annotation)
        println_code(gen, "$field_type $(field.name);")
    end

    gen.indent_level -= 1
    println_code(gen, "};")
    println_code(gen, "")
end

function gen_impl_block(gen::CodeGenerator, impl::ImplBlock)
    for method in impl.methods
        gen_fn_decl(gen, method)
    end
end

# ============================================================
# Block Code Generation
# ============================================================

function gen_block(gen::CodeGenerator, block::Block)
    for stmt in block.statements
        gen_stmt(gen, stmt)
    end
end

# ============================================================
# Statement Code Generation
# ============================================================

function gen_stmt(gen::CodeGenerator, stmt::ASTNode)
    if isa(stmt, VarDecl)
        gen_var_decl(gen, stmt)
    elseif isa(stmt, ExprStmt)
        gen_expr_stmt(gen, stmt)
    elseif isa(stmt, IfStmt)
        gen_if_stmt(gen, stmt)
    elseif isa(stmt, ForStmt)
        gen_for_stmt(gen, stmt)
    elseif isa(stmt, WhileStmt)
        gen_while_stmt(gen, stmt)
    elseif isa(stmt, LoopStmt)
        gen_loop_stmt(gen, stmt)
    elseif isa(stmt, MatchStmt)
        gen_match_stmt(gen, stmt)
    elseif isa(stmt, ReturnStmt)
        gen_return_stmt(gen, stmt)
    elseif isa(stmt, Block)
        gen_block(gen, stmt)
    end
end

function gen_var_decl(gen::CodeGenerator, decl::VarDecl)
    var_type = decl.type_annotation !== nothing ?
        type_to_c(decl.type_annotation) : "int"

    if decl.init !== nothing
        init_val = gen_expr(gen, decl.init)
        println_code(gen, "$var_type $(decl.name) = $init_val;")
    else
        println_code(gen, "$var_type $(decl.name);")
    end
end

function gen_expr_stmt(gen::CodeGenerator, stmt::ExprStmt)
    expr_val = gen_expr(gen, stmt.expr)
    println_code(gen, "$expr_val;")
end

function gen_if_stmt(gen::CodeGenerator, stmt::IfStmt)
    cond_val = gen_expr(gen, stmt.condition)
    println_code(gen, "if ($cond_val) {")

    gen.indent_level += 1
    gen_block(gen, stmt.then_block)
    gen.indent_level -= 1

    if stmt.else_block !== nothing
        println_code(gen, "} else {")
        gen.indent_level += 1
        gen_block(gen, stmt.else_block)
        gen.indent_level -= 1
    end

    println_code(gen, "}")
end

function gen_for_stmt(gen::CodeGenerator, stmt::ForStmt)
    # Range 형태로 처리
    iter_expr = gen_expr(gen, stmt.iter_expr)

    # 간단화: 0..N 형태만 지원
    println_code(gen, "for (int $(stmt.var) = 0; $(stmt.var) < $iter_expr; $(stmt.var)++) {")

    gen.indent_level += 1
    gen_block(gen, stmt.body)
    gen.indent_level -= 1

    println_code(gen, "}")
end

function gen_while_stmt(gen::CodeGenerator, stmt::WhileStmt)
    cond_val = gen_expr(gen, stmt.condition)
    println_code(gen, "while ($cond_val) {")

    gen.indent_level += 1
    gen_block(gen, stmt.body)
    gen.indent_level -= 1

    println_code(gen, "}")
end

function gen_loop_stmt(gen::CodeGenerator, stmt::LoopStmt)
    println_code(gen, "while (1) {")

    gen.indent_level += 1
    gen_block(gen, stmt.body)
    gen.indent_level -= 1

    println_code(gen, "}")
end

function gen_match_stmt(gen::CodeGenerator, stmt::MatchStmt)
    # 간단화: if-else chain으로 변환
    expr_val = gen_expr(gen, stmt.expr)

    for (i, arm) in enumerate(stmt.arms)
        if i == 1
            println_code(gen, "if ($(expr_val) == $(arm.pattern)) {")
        else
            println_code(gen, "} else if ($(expr_val) == $(arm.pattern)) {")
        end

        gen.indent_level += 1
        arm_val = gen_expr(gen, arm.expr)
        println_code(gen, arm_val)
        gen.indent_level -= 1
    end

    println_code(gen, "}")
end

function gen_return_stmt(gen::CodeGenerator, stmt::ReturnStmt)
    if stmt.value !== nothing
        val = gen_expr(gen, stmt.value)
        println_code(gen, "return $val;")
    else
        println_code(gen, "return;")
    end
end

# ============================================================
# Expression Code Generation
# ============================================================

function gen_expr(gen::CodeGenerator, expr::Expr)::String
    if isa(expr, Literal)
        return gen_literal(expr)
    elseif isa(expr, Identifier)
        return expr.name
    elseif isa(expr, BinaryOp)
        return gen_binary_op(gen, expr)
    elseif isa(expr, UnaryOp)
        return gen_unary_op(gen, expr)
    elseif isa(expr, Call)
        return gen_call(gen, expr)
    elseif isa(expr, MethodCall)
        return gen_method_call(gen, expr)
    elseif isa(expr, FieldAccess)
        return gen_field_access(gen, expr)
    elseif isa(expr, Index)
        return gen_index(gen, expr)
    elseif isa(expr, Cast)
        return gen_cast(gen, expr)
    elseif isa(expr, Range)
        return gen_range(expr)
    elseif isa(expr, ArrayLiteral)
        return gen_array_literal(gen, expr)
    else
        return "0"
    end
end

function gen_literal(lit::Literal)::String
    if isa(lit.value, Int)
        return string(lit.value)
    elseif isa(lit.value, Float)
        return string(lit.value)
    elseif isa(lit.value, String)
        # Escape quotes
        escaped = replace(lit.value, "\"" => "\\\"")
        return "\"$escaped\""
    elseif isa(lit.value, Char)
        return "'$(lit.value)'"
    elseif isa(lit.value, Bool)
        return lit.value ? "1" : "0"
    else
        return "0"
    end
end

function gen_binary_op(gen::CodeGenerator, op::BinaryOp)::String
    left = gen_expr(gen, op.left)
    right = gen_expr(gen, op.right)

    op_map = Dict(
        "+" => "+",
        "-" => "-",
        "*" => "*",
        "/" => "/",
        "%" => "%",
        "==" => "==",
        "!=" => "!=",
        "<" => "<",
        ">" => ">",
        "<=" => "<=",
        ">=" => ">=",
        "&&" => "&&",
        "||" => "||",
        "&" => "&",
        "|" => "|",
        "^" => "^",
        "<<" => "<<",
        ">>" => ">>",
        "=" => "="
    )

    c_op = get(op_map, op.op, "+")
    return "($left $c_op $right)"
end

function gen_unary_op(gen::CodeGenerator, op::UnaryOp)::String
    operand = gen_expr(gen, op.operand)

    if op.op == "-"
        return "(-$operand)"
    elseif op.op == "!"
        return "(!$operand)"
    elseif op.op == "&"
        return "(&$operand)"
    elseif op.op == "*"
        return "(*$operand)"
    elseif op.op == "~"
        return "(~$operand)"
    else
        return operand
    end
end

function gen_call(gen::CodeGenerator, call::Call)::String
    func_name = isa(call.func, String) ? call.func : "unknown_func"
    args = [gen_expr(gen, arg) for arg in call.args]
    arg_str = join(args, ", ")

    return "$func_name($arg_str)"
end

function gen_method_call(gen::CodeGenerator, call::MethodCall)::String
    obj = gen_expr(gen, call.object)
    args = [gen_expr(gen, arg) for arg in call.args]
    arg_str = join(args, ", ")

    return "$obj.$(call.method)($arg_str)"
end

function gen_field_access(gen::CodeGenerator, access::FieldAccess)::String
    obj = gen_expr(gen, access.object)
    return "$obj.$(access.field)"
end

function gen_index(gen::CodeGenerator, index::Index)::String
    array = gen_expr(gen, index.array)
    idx = gen_expr(gen, index.index)

    return "$array[$idx]"
end

function gen_cast(gen::CodeGenerator, cast::Cast)::String
    expr = gen_expr(gen, cast.expr)
    target_type = type_to_c(cast.target_type)

    return "(($target_type)$expr)"
end

function gen_range(r::Range)::String
    if r.end !== nothing
        return "$(r.end)"  # 간단화: end 값만 반환 (for loop에서 사용)
    else
        return "1000"  # 기본값
    end
end

function gen_array_literal(gen::CodeGenerator, lit::ArrayLiteral)::String
    elements = [gen_expr(gen, elem) for elem in lit.elements]
    elem_str = join(elements, ", ")

    return "{$elem_str}"
end

end  # module MinRustCodeGen
