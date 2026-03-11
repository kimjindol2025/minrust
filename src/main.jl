# MinRust - Main Entry Point

include("tokenizer.jl")
include("ast.jl")
include("parser.jl")
include("type_checker.jl")
include("codegen.jl")
include("compiler.jl")

using .MinRustTokenizer
using .MinRustAST
using .MinRustParser
using .MinRustTypeChecker
using .MinRustCodeGen
using .MinRustCompiler

# ============================================================
# Example Programs
# ============================================================

const EXAMPLE_1 = """
fn main() {
    let x = 42;
    let y = x + 10;
}
"""

const EXAMPLE_2 = """
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(10, 20);
}
"""

const EXAMPLE_3 = """
fn main() {
    let sum = 0;
    for i in 0..5 {
        let sum = sum + i;
    }
}
"""

# ============================================================
# Main Function
# ============================================================

function main()
    println("🦀 MinRust Compiler v0.1")
    println("═══════════════════════════════════════")
    println()

    # Compile Example 1
    println("📝 Compiling Example 1:")
    println(EXAMPLE_1)
    println()

    result1 = MinRustCompiler.compile(EXAMPLE_1)
    print_compilation_stats(result1)

    if result1.success && result1.c_code !== nothing
        println("\n📄 Generated C Code:")
        println("─────────────────────────────────────")
        println(result1.c_code)
        println("─────────────────────────────────────")
    end

    println()
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end
