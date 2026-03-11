# MinRust Type Checker - Semantic Analysis
# 타입 검증 및 의미 분석

module MinRustTypeChecker

using ..MinRustAST

export TypeCheckResult, check_types

# ============================================================
# Type Checking Result
# ============================================================

struct TypeCheckError <: Exception
    message::String
    line::Int
    column::Int
end

struct TypeCheckResult
    success::Bool
    errors::Vector{TypeCheckError}
    symbol_table::Dict{String, RustType}
    function_table::Dict{String, FunctionType}
end

# ============================================================
# Type Checker Structure
# ============================================================

mutable struct TypeChecker
    symbol_table::Vector{Dict{String, RustType}}  # Scope stack
    function_table::Dict{String, FunctionType}
    errors::Vector{TypeCheckError}
    current_function::Union{String, Nothing}
end

function TypeChecker()
    return TypeChecker(
        [Dict{String, RustType}()],
        Dict{String, FunctionType}(),
        TypeCheckError[],
        nothing
    )
end

# ============================================================
# Scope Management
# ============================================================

function push_scope!(checker::TypeChecker)
    push!(checker.symbol_table, Dict{String, RustType}())
end

function pop_scope!(checker::TypeChecker)
    if length(checker.symbol_table) > 1
        pop!(checker.symbol_table)
    end
end

function declare_symbol!(checker::TypeChecker, name::String, type::RustType)
    current_scope = checker.symbol_table[end]
    if haskey(current_scope, name)
        push!(checker.errors, TypeCheckError("Symbol $name already declared", -1, -1))
        return false
    end
    current_scope[name] = type
    return true
end

function lookup_symbol(checker::TypeChecker, name::String)::Union{RustType, Nothing}
    for scope in reverse(checker.symbol_table)
        if haskey(scope, name)
            return scope[name]
        end
    end
    return nothing
end

# ============================================================
# Type Compatibility & Operations
# ============================================================

function types_equal(t1::RustType, t2::RustType)::Bool
    if isa(t1, PrimitiveType) && isa(t2, PrimitiveType)
        return t1.name == t2.name
    elseif isa(t1, ReferenceType) && isa(t2, ReferenceType)
        return t1.is_mutable == t2.is_mutable && types_equal(t1.inner, t2.inner)
    elseif isa(t1, ArrayType) && isa(t2, ArrayType)
        return types_equal(t1.element_type, t2.element_type)
    elseif isa(t1, StructType) && isa(t2, StructType)
        return t1.name == t2.name
    elseif isa(t1, VecType) && isa(t2, VecType)
        return types_equal(t1.element_type, t2.element_type)
    else
        return false
    end
end

function is_compatible(from::RustType, to::RustType)::Bool
    # 정확한 일치
    if types_equal(from, to)
        return true
    end

    # &T에서 T로는 불가능 (명시적 deref 필요)
    # String은 &str로 자동 변환 가능 (simplified)

    return false
end

function binary_op_result_type(left::RustType, right::RustType, op::String)::Union{RustType, Nothing}
    if !types_equal(left, right)
        return nothing
    end

    # 산술 연산
    if op in ["+", "-", "*", "/", "%"]
        if isa(left, PrimitiveType) && left.name in ["i32", "i64", "u32", "u64", "f64"]
            return left
        end
        return nothing
    end

    # 비교 연산 (bool 반환)
    if op in ["==", "!=", "<", ">", "<=", ">="]
        return PrimitiveType("bool")
    end

    # 논리 연산 (bool)
    if op in ["&&", "||"]
        if isa(left, PrimitiveType) && left.name == "bool"
            return PrimitiveType("bool")
        end
        return nothing
    end

    # 대입 연산
    if op == "="
        return left
    end

    return nothing
end

function unary_op_result_type(op::String, operand::RustType)::Union{RustType, Nothing}
    if op == "-"
        if isa(operand, PrimitiveType) && operand.name in ["i32", "i64", "f64"]
            return operand
        end
        return nothing
    elseif op == "!"
        if isa(operand, PrimitiveType) && operand.name == "bool"
            return PrimitiveType("bool")
        end
        return nothing
    elseif op == "&"
        return ReferenceType(operand, false)
    elseif op == "*"
        if isa(operand, ReferenceType)
            return operand.inner
        end
        return nothing
    end

    return nothing
end

# ============================================================
# Main Type Checking
# ============================================================

function check_types(program::Program)::TypeCheckResult
    checker = TypeChecker()

    # 전체 함수 수집 (forward declaration)
    collect_functions!(checker, program)

    # 각 항목 검사
    for item in program.items
        check_item!(checker, item)
    end

    return TypeCheckResult(
        isempty(checker.errors),
        checker.errors,
        checker.symbol_table[1],
        checker.function_table
    )
end

function collect_functions!(checker::TypeChecker, program::Program)
    for item in program.items
        if isa(item, FnDecl)
            param_types = [p.type_annotation for p in item.params]
            return_type = item.return_type !== nothing ? item.return_type : PrimitiveType("void")
            fn_type = FunctionType(param_types, return_type)
            checker.function_table[item.name] = fn_type
        end
    end
end

function check_item!(checker::TypeChecker, item::ASTNode)
    if isa(item, FnDecl)
        check_fn_decl!(checker, item)
    elseif isa(item, StructDecl)
        check_struct_decl!(checker, item)
    elseif isa(item, ImplBlock)
        check_impl_block!(checker, item)
    end
end

function check_fn_decl!(checker::TypeChecker, fn::FnDecl)
    push_scope!(checker)
    checker.current_function = fn.name

    # 매개변수 등록
    for param in fn.params
        declare_symbol!(checker, param.name, param.type_annotation)
    end

    # 함수 본체 검사
    check_block!(checker, fn.body)

    pop_scope!(checker)
    checker.current_function = nothing
end

function check_struct_decl!(checker::TypeChecker, struct_decl::StructDecl)
    # 간단화: 구조체 필드는 별도로 검증하지 않음
end

function check_impl_block!(checker::TypeChecker, impl::ImplBlock)
    for method in impl.methods
        check_fn_decl!(checker, method)
    end
end

function check_block!(checker::TypeChecker, block::Block)
    for stmt in block.statements
        check_stmt!(checker, stmt)
    end
end

# ============================================================
# Statement Checking
# ============================================================

function check_stmt!(checker::TypeChecker, stmt::ASTNode)
    if isa(stmt, VarDecl)
        check_var_decl!(checker, stmt)
    elseif isa(stmt, ExprStmt)
        check_expr_stmt!(checker, stmt)
    elseif isa(stmt, IfStmt)
        check_if_stmt!(checker, stmt)
    elseif isa(stmt, ForStmt)
        check_for_stmt!(checker, stmt)
    elseif isa(stmt, WhileStmt)
        check_while_stmt!(checker, stmt)
    elseif isa(stmt, LoopStmt)
        check_loop_stmt!(checker, stmt)
    elseif isa(stmt, MatchStmt)
        check_match_stmt!(checker, stmt)
    elseif isa(stmt, ReturnStmt)
        check_return_stmt!(checker, stmt)
    elseif isa(stmt, Block)
        push_scope!(checker)
        check_block!(checker, stmt)
        pop_scope!(checker)
    end
end

function check_var_decl!(checker::TypeChecker, decl::VarDecl)
    # 초기값이 있으면 타입 추론
    if decl.init !== nothing
        init_type = check_expr!(checker, decl.init)

        if decl.type_annotation !== nothing
            # 명시적 타입 지정
            if !is_compatible(init_type, decl.type_annotation)
                push!(checker.errors, TypeCheckError(
                    "Type mismatch: got $init_type, expected $(decl.type_annotation)", -1, -1))
            end
            declare_symbol!(checker, decl.name, decl.type_annotation)
        else
            # 타입 추론
            declare_symbol!(checker, decl.name, init_type)
        end
    else
        # 초기값 없으면 타입 지정 필수
        if decl.type_annotation !== nothing
            declare_symbol!(checker, decl.name, decl.type_annotation)
        else
            push!(checker.errors, TypeCheckError(
                "Variable $(decl.name) must have explicit type or initializer", -1, -1))
        end
    end
end

function check_expr_stmt!(checker::TypeChecker, stmt::ExprStmt)
    check_expr!(checker, stmt.expr)
end

function check_if_stmt!(checker::TypeChecker, stmt::IfStmt)
    cond_type = check_expr!(checker, stmt.condition)

    if !isa(cond_type, PrimitiveType) || cond_type.name != "bool"
        push!(checker.errors, TypeCheckError(
            "Condition must be bool, got $cond_type", -1, -1))
    end

    push_scope!(checker)
    check_block!(checker, stmt.then_block)
    pop_scope!(checker)

    if stmt.else_block !== nothing
        push_scope!(checker)
        check_block!(checker, stmt.else_block)
        pop_scope!(checker)
    end
end

function check_for_stmt!(checker::TypeChecker, stmt::ForStmt)
    iter_type = check_expr!(checker, stmt.iter_expr)

    push_scope!(checker)
    # for var in iterable 형태에서 var은 element type
    if isa(iter_type, ArrayType)
        declare_symbol!(checker, stmt.var, iter_type.element_type)
    elseif isa(iter_type, VecType)
        declare_symbol!(checker, stmt.var, iter_type.element_type)
    elseif isa(iter_type, Range)
        declare_symbol!(checker, stmt.var, PrimitiveType("i32"))  # Range yields i32
    else
        push!(checker.errors, TypeCheckError(
            "Cannot iterate over $iter_type", -1, -1))
    end

    check_block!(checker, stmt.body)
    pop_scope!(checker)
end

function check_while_stmt!(checker::TypeChecker, stmt::WhileStmt)
    cond_type = check_expr!(checker, stmt.condition)

    if !isa(cond_type, PrimitiveType) || cond_type.name != "bool"
        push!(checker.errors, TypeCheckError(
            "While condition must be bool", -1, -1))
    end

    push_scope!(checker)
    check_block!(checker, stmt.body)
    pop_scope!(checker)
end

function check_loop_stmt!(checker::TypeChecker, stmt::LoopStmt)
    push_scope!(checker)
    check_block!(checker, stmt.body)
    pop_scope!(checker)
end

function check_match_stmt!(checker::TypeChecker, stmt::MatchStmt)
    expr_type = check_expr!(checker, stmt.expr)

    for arm in stmt.arms
        push_scope!(checker)
        # 매칭 패턴에 따라 변수 선언 (간단화)
        check_expr!(checker, arm.expr)
        pop_scope!(checker)
    end
end

function check_return_stmt!(checker::TypeChecker, stmt::ReturnStmt)
    if stmt.value !== nothing
        check_expr!(checker, stmt.value)
    end
end

# ============================================================
# Expression Type Checking
# ============================================================

function check_expr!(checker::TypeChecker, expr::Expr)::RustType
    if isa(expr, Literal)
        return check_literal(expr)
    elseif isa(expr, Identifier)
        return check_identifier!(checker, expr)
    elseif isa(expr, BinaryOp)
        return check_binary_op!(checker, expr)
    elseif isa(expr, UnaryOp)
        return check_unary_op!(checker, expr)
    elseif isa(expr, Call)
        return check_call!(checker, expr)
    elseif isa(expr, MethodCall)
        return check_method_call!(checker, expr)
    elseif isa(expr, FieldAccess)
        return check_field_access!(checker, expr)
    elseif isa(expr, Index)
        return check_index!(checker, expr)
    elseif isa(expr, ArrayLiteral)
        return check_array_literal!(checker, expr)
    elseif isa(expr, Cast)
        return check_cast!(checker, expr)
    elseif isa(expr, Range)
        return check_range!(checker, expr)
    else
        return PrimitiveType("unknown")
    end
end

function check_literal(lit::Literal)::RustType
    if isa(lit.value, Int)
        return PrimitiveType("i32")
    elseif isa(lit.value, Float)
        return PrimitiveType("f64")
    elseif isa(lit.value, String)
        return PrimitiveType("String")
    elseif isa(lit.value, Char)
        return PrimitiveType("char")
    elseif isa(lit.value, Bool)
        return PrimitiveType("bool")
    else
        return PrimitiveType("unknown")
    end
end

function check_identifier!(checker::TypeChecker, ident::Identifier)::RustType
    sym_type = lookup_symbol(checker, ident.name)
    if sym_type === nothing
        push!(checker.errors, TypeCheckError(
            "Undefined variable: $(ident.name)", -1, -1))
        return PrimitiveType("unknown")
    end
    return sym_type
end

function check_binary_op!(checker::TypeChecker, op::BinaryOp)::RustType
    left_type = check_expr!(checker, op.left)
    right_type = check_expr!(checker, op.right)

    result_type = binary_op_result_type(left_type, right_type, op.op)

    if result_type === nothing
        push!(checker.errors, TypeCheckError(
            "Invalid binary operation: $left_type $(op.op) $right_type", -1, -1))
        return PrimitiveType("unknown")
    end

    return result_type
end

function check_unary_op!(checker::TypeChecker, op::UnaryOp)::RustType
    operand_type = check_expr!(checker, op.operand)

    result_type = unary_op_result_type(op.op, operand_type)

    if result_type === nothing
        push!(checker.errors, TypeCheckError(
            "Invalid unary operation: $(op.op) $operand_type", -1, -1))
        return PrimitiveType("unknown")
    end

    return result_type
end

function check_call!(checker::TypeChecker, call::Call)::RustType
    func_name = isa(call.func, String) ? call.func : ""
    fn_type = get(checker.function_table, func_name, nothing)

    if fn_type === nothing
        push!(checker.errors, TypeCheckError(
            "Undefined function: $func_name", -1, -1))
        return PrimitiveType("unknown")
    end

    if length(call.args) != length(fn_type.param_types)
        push!(checker.errors, TypeCheckError(
            "Function $func_name expects $(length(fn_type.param_types)) arguments, got $(length(call.args))", -1, -1))
    end

    # 인자 타입 검사
    for (i, arg) in enumerate(call.args)
        arg_type = check_expr!(checker, arg)
        if i <= length(fn_type.param_types)
            if !is_compatible(arg_type, fn_type.param_types[i])
                push!(checker.errors, TypeCheckError(
                    "Argument $i type mismatch", -1, -1))
            end
        end
    end

    return fn_type.return_type
end

function check_method_call!(checker::TypeChecker, call::MethodCall)::RustType
    obj_type = check_expr!(checker, call.object)
    # 간단화: method type checking
    return PrimitiveType("unknown")
end

function check_field_access!(checker::TypeChecker, access::FieldAccess)::RustType
    obj_type = check_expr!(checker, access.object)
    # 간단화: field type checking
    return PrimitiveType("unknown")
end

function check_index!(checker::TypeChecker, index::Index)::RustType
    array_type = check_expr!(checker, index.array)
    index_type = check_expr!(checker, index.index)

    if !isa(index_type, PrimitiveType) || index_type.name != "i32"
        push!(checker.errors, TypeCheckError(
            "Array index must be i32", -1, -1))
    end

    if isa(array_type, ArrayType)
        return array_type.element_type
    elseif isa(array_type, VecType)
        return array_type.element_type
    else
        push!(checker.errors, TypeCheckError(
            "Cannot index non-array type", -1, -1))
        return PrimitiveType("unknown")
    end
end

function check_array_literal!(checker::TypeChecker, lit::ArrayLiteral)::RustType
    if isempty(lit.elements)
        return ArrayType(PrimitiveType("unknown"), 0)
    end

    element_type = check_expr!(checker, lit.elements[1])

    for elem in lit.elements[2:end]
        elem_type = check_expr!(checker, elem)
        if !types_equal(elem_type, element_type)
            push!(checker.errors, TypeCheckError(
                "Array element type mismatch", -1, -1))
        end
    end

    return ArrayType(element_type, length(lit.elements))
end

function check_cast!(checker::TypeChecker, cast::Cast)::RustType
    expr_type = check_expr!(checker, cast.expr)
    # 간단화: cast always succeeds
    return cast.target_type
end

function check_range!(checker::TypeChecker, r::Range)::RustType
    if r.start !== nothing
        check_expr!(checker, r.start)
    end
    if r.end !== nothing
        check_expr!(checker, r.end)
    end
    return Range(r.start, r.end, r.inclusive)
end

end  # module MinRustTypeChecker
