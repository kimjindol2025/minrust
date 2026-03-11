# MinRust Compiler - Integration Module
# 모든 컴파일 단계를 통합

module MinRustCompiler

using ..MinRustTokenizer
using ..MinRustParser
using ..MinRustTypeChecker
using ..MinRustCodeGen

export compile, CompilationResult

# ============================================================
# Compilation Result
# ============================================================

struct CompilationResult
    success::Bool
    c_code::Union{String, Nothing}
    errors::Vector{String}
    token_count::Int
    ast_nodes::Int
    warnings::Vector{String}
end

function Base.show(io::IO, result::CompilationResult)
    if result.success
        println(io, "✅ Compilation Success")
        println(io, "  Tokens: $(result.token_count)")
        println(io, "  AST Nodes: $(result.ast_nodes)")
        if !isempty(result.warnings)
            println(io, "  Warnings: $(length(result.warnings))")
        end
    else
        println(io, "❌ Compilation Failed")
        println(io, "  Errors: $(length(result.errors))")
        for error in result.errors
            println(io, "    - $error")
        end
    end
end

# ============================================================
# Main Compilation Pipeline
# ============================================================

function compile(input::String)::CompilationResult
    errors = String[]
    warnings = String[]

    # Step 1: Tokenization
    try
        tokens = tokenize(input)
        if isempty(tokens)
            push!(errors, "Tokenizer produced no tokens")
            return CompilationResult(false, nothing, errors, 0, 0, warnings)
        end

        # Step 2: Parsing
        try
            parser = MinRustParser.Parser(tokens)
            program = MinRustParser.parse(tokens)

            if program === nothing || isempty(program.items)
                push!(errors, "Parser produced empty AST")
                return CompilationResult(false, nothing, errors, length(tokens), 0, warnings)
            end

            ast_node_count = count_ast_nodes(program)

            # Step 3: Type Checking
            try
                type_check_result = MinRustTypeChecker.check_types(program)

                if !type_check_result.success
                    for error in type_check_result.errors
                        push!(errors, error.message)
                    end
                    return CompilationResult(false, nothing, errors, length(tokens), ast_node_count, warnings)
                end

                # Step 4: Code Generation
                try
                    c_code = MinRustCodeGen.generate_code(program)

                    return CompilationResult(
                        true,
                        c_code,
                        errors,
                        length(tokens),
                        ast_node_count,
                        warnings
                    )

                catch e
                    push!(errors, "Code generation error: $(e)")
                    return CompilationResult(false, nothing, errors, length(tokens), ast_node_count, warnings)
                end

            catch e
                push!(errors, "Type checking error: $(e)")
                return CompilationResult(false, nothing, errors, length(tokens), ast_node_count, warnings)
            end

        catch e
            push!(errors, "Parsing error: $(e)")
            return CompilationResult(false, nothing, errors, length(tokens), 0, warnings)
        end

    catch e
        push!(errors, "Tokenization error: $(e)")
        return CompilationResult(false, nothing, errors, 0, 0, warnings)
    end
end

# ============================================================
# Helper Functions
# ============================================================

function count_ast_nodes(program::MinRustAST.Program)::Int
    count = 1  # Program itself

    for item in program.items
        count += count_node(item)
    end

    return count
end

function count_node(node::Any)::Int
    count = 1

    if isa(node, MinRustAST.FnDecl)
        count += count_node(node.body)
        for param in node.params
            count += 1
        end
    elseif isa(node, MinRustAST.Block)
        for stmt in node.statements
            count += count_node(stmt)
        end
    elseif isa(node, MinRustAST.IfStmt)
        count += count_node(node.condition)
        count += count_node(node.then_block)
        if node.else_block !== nothing
            count += count_node(node.else_block)
        end
    elseif isa(node, MinRustAST.ForStmt)
        count += count_node(node.iter_expr)
        count += count_node(node.body)
    elseif isa(node, MinRustAST.WhileStmt)
        count += count_node(node.condition)
        count += count_node(node.body)
    elseif isa(node, MinRustAST.BinaryOp)
        count += count_node(node.left)
        count += count_node(node.right)
    elseif isa(node, MinRustAST.UnaryOp)
        count += count_node(node.operand)
    elseif isa(node, MinRustAST.Call)
        for arg in node.args
            count += count_node(arg)
        end
    end

    return count
end

# ============================================================
# Compilation with Custom Options
# ============================================================

struct CompilerOptions
    optimize::Bool
    debug::Bool
    output_ir::Bool
end

function compile_with_options(input::String, options::CompilerOptions)::CompilationResult
    result = compile(input)

    if options.output_ir
        # 미래: IR 출력 옵션
    end

    if options.optimize
        # 미래: 최적화 적용 (Phase 6)
    end

    return result
end

# ============================================================
# Compilation Pipeline Statistics
# ============================================================

function print_compilation_stats(result::CompilationResult)
    println("═══════════════════════════════════════")
    println("MinRust Compilation Statistics")
    println("═══════════════════════════════════════")
    println("Status: $(result.success ? "✅ Success" : "❌ Failed")")
    println("Tokens: $(result.token_count)")
    println("AST Nodes: $(result.ast_nodes)")

    if !isempty(result.errors)
        println("\nErrors: $(length(result.errors))")
        for (i, error) in enumerate(result.errors)
            println("  [$i] $error")
        end
    end

    if !isempty(result.warnings)
        println("\nWarnings: $(length(result.warnings))")
        for (i, warning) in enumerate(result.warnings)
            println("  [$i] $warning")
        end
    end

    if result.success && result.c_code !== nothing
        c_lines = length(split(result.c_code, '\n'))
        println("\nGenerated C Code:")
        println("  Lines: $c_lines")
    end

    println("═══════════════════════════════════════")
end

end  # module MinRustCompiler
