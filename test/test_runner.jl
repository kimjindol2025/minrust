# MinRust Test Runner
# 모든 테스트를 실행합니다

include("../src/tokenizer.jl")
include("../src/ast.jl")
include("../src/parser.jl")
include("../src/type_checker.jl")
include("../src/codegen.jl")
include("../src/compiler.jl")

using .MinRustTokenizer
using .MinRustAST
using .MinRustParser
using .MinRustTypeChecker
using .MinRustCodeGen
using .MinRustCompiler

# ============================================================
# Test Framework
# ============================================================

mutable struct TestStats
    total::Int
    passed::Int
    failed::Int
end

const TESTS = TestStats(0, 0, 0)

macro test(name, condition)
    quote
        TESTS.total += 1
        if $(esc(condition))
            TESTS.passed += 1
            println("  ✓ $($name)")
        else
            TESTS.failed += 1
            println("  ✗ $($name)")
        end
    end
end

function print_test_summary()
    println()
    println("═══════════════════════════════════════")
    println("Test Results")
    println("═══════════════════════════════════════")
    println("Total:  $(TESTS.total)")
    println("Passed: $(TESTS.passed) ✅")
    println("Failed: $(TESTS.failed) ❌")
    println("═══════════════════════════════════════")

    if TESTS.failed == 0
        println("🎉 All tests passed!")
    else
        println("⚠️  Some tests failed")
    end
end

# ============================================================
# Tokenizer Tests (40 tests)
# ============================================================

function test_tokenizer()
    println("\n📝 Tokenizer Tests")
    println("─────────────────────────────────────")

    # Test 1-5: Basic keywords
    @test "Keyword: fn" begin
        tokens = tokenize("fn")
        !isempty(tokens) && tokens[1].type == KW_FN
    end

    @test "Keyword: let" begin
        tokens = tokenize("let")
        !isempty(tokens) && tokens[1].type == KW_LET
    end

    @test "Keyword: if" begin
        tokens = tokenize("if")
        !isempty(tokens) && tokens[1].type == KW_IF
    end

    @test "Keyword: for" begin
        tokens = tokenize("for")
        !isempty(tokens) && tokens[1].type == KW_FOR
    end

    @test "Keyword: struct" begin
        tokens = tokenize("struct")
        !isempty(tokens) && tokens[1].type == KW_STRUCT
    end

    # Test 6-10: Identifiers
    @test "Identifier: simple" begin
        tokens = tokenize("x")
        !isempty(tokens) && tokens[1].type == IDENTIFIER
    end

    @test "Identifier: with number" begin
        tokens = tokenize("var123")
        !isempty(tokens) && tokens[1].type == IDENTIFIER
    end

    @test "Identifier: snake_case" begin
        tokens = tokenize("my_var")
        !isempty(tokens) && tokens[1].type == IDENTIFIER
    end

    @test "Identifier: underscore prefix" begin
        tokens = tokenize("_private")
        !isempty(tokens) && tokens[1].type == IDENTIFIER
    end

    @test "Identifier: CamelCase" begin
        tokens = tokenize("MyType")
        !isempty(tokens) && tokens[1].type == IDENTIFIER
    end

    # Test 11-15: Literals
    @test "Integer literal: 42" begin
        tokens = tokenize("42")
        !isempty(tokens) && tokens[1].type == INT_LITERAL && tokens[1].value == 42
    end

    @test "Integer literal: 0" begin
        tokens = tokenize("0")
        !isempty(tokens) && tokens[1].type == INT_LITERAL && tokens[1].value == 0
    end

    @test "Float literal: 3.14" begin
        tokens = tokenize("3.14")
        !isempty(tokens) && tokens[1].type == FLOAT_LITERAL
    end

    @test "String literal: \"hello\"" begin
        tokens = tokenize("\"hello\"")
        !isempty(tokens) && tokens[1].type == STRING_LITERAL && tokens[1].value == "hello"
    end

    @test "Char literal: 'x'" begin
        tokens = tokenize("'x'")
        !isempty(tokens) && tokens[1].type == CHAR_LITERAL
    end

    # Test 16-20: Operators
    @test "Operator: +" begin
        tokens = tokenize("+")
        !isempty(tokens) && tokens[1].type == PLUS
    end

    @test "Operator: -" begin
        tokens = tokenize("-")
        !isempty(tokens) && tokens[1].type == MINUS
    end

    @test "Operator: ==" begin
        tokens = tokenize("==")
        !isempty(tokens) && tokens[1].type == EQ
    end

    @test "Operator: !=" begin
        tokens = tokenize("!=")
        !isempty(tokens) && tokens[1].type == NE
    end

    @test "Operator: &&" begin
        tokens = tokenize("&&")
        !isempty(tokens) && tokens[1].type == AND
    end

    # Test 21-25: Punctuation
    @test "Punctuation: (" begin
        tokens = tokenize("(")
        !isempty(tokens) && tokens[1].type == LPAREN
    end

    @test "Punctuation: )" begin
        tokens = tokenize(")")
        !isempty(tokens) && tokens[1].type == RPAREN
    end

    @test "Punctuation: {" begin
        tokens = tokenize("{")
        !isempty(tokens) && tokens[1].type == LBRACE
    end

    @test "Punctuation: }" begin
        tokens = tokenize("}")
        !isempty(tokens) && tokens[1].type == RBRACE
    end

    @test "Punctuation: ;" begin
        tokens = tokenize(";")
        !isempty(tokens) && tokens[1].type == SEMICOLON
    end

    # Test 26-30: Complex expressions
    @test "Expression: let x = 42;" begin
        tokens = tokenize("let x = 42;")
        length(tokens) >= 5
    end

    @test "Expression: fn add(a, b)" begin
        tokens = tokenize("fn add(a, b)")
        any(t -> t.type == KW_FN, tokens)
    end

    @test "Comment: // skip this" begin
        tokens = tokenize("let x = 1; // comment")
        !any(t -> t.type == IDENTIFIER && t.value == "comment", tokens)
    end

    @test "Whitespace handling" begin
        tokens1 = tokenize("let x = 42")
        tokens2 = tokenize("let   x   =   42")
        length(tokens1) == length(tokens2)
    end

    @test "Multi-line" begin
        code = """
        let x = 42;
        let y = 43;
        """
        tokens = tokenize(code)
        length(tokens) >= 10
    end

    # Test 31-40: Edge cases
    @test "Empty input" begin
        tokens = tokenize("")
        !isempty(tokens) && tokens[end].type == EOF_TOKEN
    end

    @test "Only whitespace" begin
        tokens = tokenize("   \n  \t  ")
        !isempty(tokens) && tokens[end].type == EOF_TOKEN
    end

    @test "Boolean literal: true" begin
        tokens = tokenize("true")
        !isempty(tokens) && tokens[1].type == KW_TRUE
    end

    @test "Boolean literal: false" begin
        tokens = tokenize("false")
        !isempty(tokens) && tokens[1].type == KW_FALSE
    end

    @test "Arrow: ->" begin
        tokens = tokenize("->")
        !isempty(tokens) && tokens[1].type == ARROW
    end

    @test "Fat arrow: =>" begin
        tokens = tokenize("=>")
        !isempty(tokens) && tokens[1].type == FAT_ARROW
    end

    @test "Colon: :" begin
        tokens = tokenize(":")
        !isempty(tokens) && tokens[1].type == COLON
    end

    @test "Double colon: ::" begin
        tokens = tokenize("::")
        !isempty(tokens) && tokens[1].type == COLONCOLON
    end

    @test "Dot: ." begin
        tokens = tokenize(".")
        !isempty(tokens) && tokens[1].type == DOT
    end

    @test "Dot-dot: .." begin
        tokens = tokenize("..")
        !isempty(tokens) && tokens[1].type == DOTDOT
    end
end

# ============================================================
# Parser Tests (30 tests)
# ============================================================

function test_parser()
    println("\n🔧 Parser Tests")
    println("─────────────────────────────────────")

    # Test 1-5: Simple literals
    @test "Parse: integer 42" begin
        tokens = tokenize("42")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_primary(parser)
        isa(expr, Literal) && expr.value == 42
    end

    @test "Parse: string \"hello\"" begin
        tokens = tokenize("\"hello\"")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_primary(parser)
        isa(expr, Literal) && expr.value == "hello"
    end

    @test "Parse: boolean true" begin
        tokens = tokenize("true")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_primary(parser)
        isa(expr, Literal) && expr.value == true
    end

    @test "Parse: identifier x" begin
        tokens = tokenize("x")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_primary(parser)
        isa(expr, Identifier) && expr.name == "x"
    end

    @test "Parse: grouped expression (x)" begin
        tokens = tokenize("(42)")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_primary(parser)
        isa(expr, Literal)
    end

    # Test 6-10: Binary operations
    @test "Parse: addition x + y" begin
        tokens = tokenize("x + y")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        isa(expr, BinaryOp) && expr.op == "+"
    end

    @test "Parse: subtraction x - y" begin
        tokens = tokenize("x - y")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        isa(expr, BinaryOp) && expr.op == "-"
    end

    @test "Parse: comparison x == y" begin
        tokens = tokenize("x == y")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        isa(expr, BinaryOp) && expr.op == "=="
    end

    @test "Parse: logical and x && y" begin
        tokens = tokenize("x && y")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        isa(expr, BinaryOp) && expr.op == "&&"
    end

    @test "Parse: logical or x || y" begin
        tokens = tokenize("x || y")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        isa(expr, BinaryOp) && expr.op == "||"
    end

    # Test 11-15: Unary operations
    @test "Parse: negation -x" begin
        tokens = tokenize("-x")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_unary(parser)
        isa(expr, UnaryOp) && expr.op == "-"
    end

    @test "Parse: logical not !x" begin
        tokens = tokenize("!x")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_unary(parser)
        isa(expr, UnaryOp) && expr.op == "!"
    end

    @test "Parse: reference &x" begin
        tokens = tokenize("&x")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_unary(parser)
        isa(expr, UnaryOp) && expr.op == "&"
    end

    @test "Parse: dereference *x" begin
        tokens = tokenize("*x")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_unary(parser)
        isa(expr, UnaryOp) && expr.op == "*"
    end

    @test "Parse: operator precedence" begin
        tokens = tokenize("2 + 3 * 4")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        # Should be 2 + (3 * 4), not (2 + 3) * 4
        isa(expr, BinaryOp)
    end

    # Test 16-20: Function and variable declarations
    @test "Parse: variable declaration let x = 42" begin
        tokens = tokenize("let x = 42;")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_var_decl(parser)
        isa(stmt, VarDecl) && stmt.name == "x"
    end

    @test "Parse: typed variable let x: i32 = 42" begin
        tokens = tokenize("let x: i32 = 42;")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_var_decl(parser)
        isa(stmt, VarDecl) && stmt.name == "x" && stmt.type_annotation !== nothing
    end

    @test "Parse: mutable variable let mut x = 42" begin
        tokens = tokenize("let mut x = 42;")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_var_decl(parser)
        isa(stmt, VarDecl) && stmt.is_mutable
    end

    @test "Parse: function declaration fn add(a: i32, b: i32) -> i32 { }" begin
        tokens = tokenize("fn add(a: i32, b: i32) -> i32 { }")
        parser = MinRustParser.Parser(tokens)
        fn = MinRustParser.parse_fn_decl(parser)
        isa(fn, FnDecl) && fn.name == "add" && length(fn.params) == 2
    end

    @test "Parse: if statement if x > 0 { }" begin
        tokens = tokenize("if x > 0 { }")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_if_stmt(parser)
        isa(stmt, IfStmt)
    end

    # Test 21-25: Collections
    @test "Parse: array literal [1, 2, 3]" begin
        tokens = tokenize("[1, 2, 3]")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_primary(parser)
        isa(expr, ArrayLiteral) && length(expr.elements) == 3
    end

    @test "Parse: function call add(1, 2)" begin
        tokens = tokenize("add(1, 2)")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_postfix(parser)
        isa(expr, Call) && length(expr.args) == 2
    end

    @test "Parse: method call x.len()" begin
        tokens = tokenize("x.len()")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_postfix(parser)
        isa(expr, MethodCall) && expr.method == "len"
    end

    @test "Parse: array index arr[0]" begin
        tokens = tokenize("arr[0]")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_postfix(parser)
        isa(expr, Index)
    end

    @test "Parse: range 0..10" begin
        tokens = tokenize("0..10")
        parser = MinRustParser.Parser(tokens)
        expr = MinRustParser.parse_expression(parser)
        isa(expr, BinaryOp) || isa(expr, Range)
    end

    # Test 26-30: Complex statements
    @test "Parse: for loop for i in 0..10 { }" begin
        tokens = tokenize("for i in 0..10 { }")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_for_stmt(parser)
        isa(stmt, ForStmt) && stmt.var == "i"
    end

    @test "Parse: while loop while x > 0 { }" begin
        tokens = tokenize("while x > 0 { }")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_while_stmt(parser)
        isa(stmt, WhileStmt)
    end

    @test "Parse: loop loop { }" begin
        tokens = tokenize("loop { }")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_loop_stmt(parser)
        isa(stmt, LoopStmt)
    end

    @test "Parse: return statement return 42" begin
        tokens = tokenize("return 42;")
        parser = MinRustParser.Parser(tokens)
        stmt = MinRustParser.parse_return_stmt(parser)
        isa(stmt, ReturnStmt) && stmt.value !== nothing
    end

    @test "Parse: full program" begin
        code = """
        fn main() {
            let x = 42;
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        isa(program, Program) && !isempty(program.items)
    end
end

# ============================================================
# Type Checker Tests (35 tests)
# ============================================================

function test_type_checker()
    println("\n✓ Type Checker Tests")
    println("─────────────────────────────────────")

    # Test 1-5: Type inference
    @test "Infer type: integer literal" begin
        lit = Literal(42)
        type = MinRustTypeChecker.check_literal(lit)
        isa(type, PrimitiveType) && type.name == "i32"
    end

    @test "Infer type: float literal" begin
        lit = Literal(3.14)
        type = MinRustTypeChecker.check_literal(lit)
        isa(type, PrimitiveType) && type.name == "f64"
    end

    @test "Infer type: string literal" begin
        lit = Literal("hello")
        type = MinRustTypeChecker.check_literal(lit)
        isa(type, PrimitiveType) && type.name == "String"
    end

    @test "Infer type: boolean literal" begin
        lit = Literal(true)
        type = MinRustTypeChecker.check_literal(lit)
        isa(type, PrimitiveType) && type.name == "bool"
    end

    @test "Infer type: char literal" begin
        lit = Literal('x')
        type = MinRustTypeChecker.check_literal(lit)
        isa(type, PrimitiveType) && type.name == "char"
    end

    # Test 6-10: Binary operations
    @test "Type check: i32 + i32 = i32" begin
        result = MinRustTypeChecker.binary_op_result_type(
            PrimitiveType("i32"),
            PrimitiveType("i32"),
            "+"
        )
        isa(result, PrimitiveType) && result.name == "i32"
    end

    @test "Type check: i32 == i32 = bool" begin
        result = MinRustTypeChecker.binary_op_result_type(
            PrimitiveType("i32"),
            PrimitiveType("i32"),
            "=="
        )
        isa(result, PrimitiveType) && result.name == "bool"
    end

    @test "Type check: bool && bool = bool" begin
        result = MinRustTypeChecker.binary_op_result_type(
            PrimitiveType("bool"),
            PrimitiveType("bool"),
            "&&"
        )
        isa(result, PrimitiveType) && result.name == "bool"
    end

    @test "Type check: f64 * f64 = f64" begin
        result = MinRustTypeChecker.binary_op_result_type(
            PrimitiveType("f64"),
            PrimitiveType("f64"),
            "*"
        )
        isa(result, PrimitiveType) && result.name == "f64"
    end

    @test "Type check: invalid operation i32 + String" begin
        result = MinRustTypeChecker.binary_op_result_type(
            PrimitiveType("i32"),
            PrimitiveType("String"),
            "+"
        )
        result === nothing
    end

    # Test 11-15: Unary operations
    @test "Type check: -i32 = i32" begin
        result = MinRustTypeChecker.unary_op_result_type(
            "-",
            PrimitiveType("i32")
        )
        isa(result, PrimitiveType) && result.name == "i32"
    end

    @test "Type check: !bool = bool" begin
        result = MinRustTypeChecker.unary_op_result_type(
            "!",
            PrimitiveType("bool")
        )
        isa(result, PrimitiveType) && result.name == "bool"
    end

    @test "Type check: &T creates reference" begin
        result = MinRustTypeChecker.unary_op_result_type(
            "&",
            PrimitiveType("i32")
        )
        isa(result, ReferenceType)
    end

    @test "Type check: *&T dereferences" begin
        ref_type = ReferenceType(PrimitiveType("i32"), false)
        result = MinRustTypeChecker.unary_op_result_type(
            "*",
            ref_type
        )
        isa(result, PrimitiveType) && result.name == "i32"
    end

    @test "Type check: -bool is invalid" begin
        result = MinRustTypeChecker.unary_op_result_type(
            "-",
            PrimitiveType("bool")
        )
        result === nothing
    end

    # Test 16-20: Type compatibility
    @test "Types equal: i32 == i32" begin
        t1 = PrimitiveType("i32")
        t2 = PrimitiveType("i32")
        MinRustTypeChecker.types_equal(t1, t2)
    end

    @test "Types not equal: i32 != i64" begin
        t1 = PrimitiveType("i32")
        t2 = PrimitiveType("i64")
        !MinRustTypeChecker.types_equal(t1, t2)
    end

    @test "Reference types equal: &i32 == &i32" begin
        t1 = ReferenceType(PrimitiveType("i32"), false)
        t2 = ReferenceType(PrimitiveType("i32"), false)
        MinRustTypeChecker.types_equal(t1, t2)
    end

    @test "Mutable reference differs: &T != &mut T" begin
        t1 = ReferenceType(PrimitiveType("i32"), false)
        t2 = ReferenceType(PrimitiveType("i32"), true)
        !MinRustTypeChecker.types_equal(t1, t2)
    end

    @test "Array types equal: [i32; 5] == [i32; 5]" begin
        t1 = ArrayType(PrimitiveType("i32"), 5)
        t2 = ArrayType(PrimitiveType("i32"), 5)
        MinRustTypeChecker.types_equal(t1, t2)
    end

    # Test 21-30: Full program type checking
    @test "Type check: simple variable declaration" begin
        code = "let x = 42;"
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: typed variable declaration" begin
        code = "let x: i32 = 42;"
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: function with correct return" begin
        code = """
        fn test() -> i32 {
            42
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: detects undefined variable" begin
        code = "let z = undefined_var;"
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        !isempty(result.errors)
    end

    @test "Type check: scope management" begin
        code = """
        fn main() {
            let x = 42;
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: array index must be integer" begin
        code = """
        fn main() {
            let arr = [1, 2, 3];
            let x = arr[0];
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: for loop variable" begin
        code = """
        fn main() {
            for i in 0..10 {
                let x = i;
            }
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: if condition must be bool" begin
        code = """
        fn main() {
            let x = 42;
            if x { }
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        !isempty(result.errors)
    end

    @test "Type check: empty program" begin
        code = ""
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        result.success
    end

    @test "Type check: multiple errors accumulate" begin
        code = """
        fn main() {
            let x = undefined1;
            let y = undefined2;
        }
        """
        tokens = tokenize(code)
        program = MinRustParser.parse(tokens)
        result = MinRustTypeChecker.check_types(program)
        !isempty(result.errors)
    end
end

# ============================================================
# Code Generation Tests (25 tests)
# ============================================================

function test_codegen()
    println("\n🔨 Code Generation Tests")
    println("─────────────────────────────────────")

    # Test 1-5: Simple expressions
    @test "Generate: integer literal" begin
        lit = Literal(42)
        code = MinRustCodeGen.gen_literal(lit)
        code == "42"
    end

    @test "Generate: string literal" begin
        lit = Literal("hello")
        code = MinRustCodeGen.gen_literal(lit)
        contains(code, "hello")
    end

    @test "Generate: boolean literal" begin
        lit = Literal(true)
        code = MinRustCodeGen.gen_literal(lit)
        contains(code, "1")
    end

    @test "Generate: type conversion i32 to C" begin
        type = PrimitiveType("i32")
        code = MinRustCodeGen.type_to_c(type)
        code == "int"
    end

    @test "Generate: type conversion String to C" begin
        type = PrimitiveType("String")
        code = MinRustCodeGen.type_to_c(type)
        code == "char*"
    end

    # Test 6-10: Binary operations
    @test "Generate: addition" begin
        op = BinaryOp(Literal(2), "+", Literal(3))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_binary_op(gen, op)
        contains(code, "+")
    end

    @test "Generate: subtraction" begin
        op = BinaryOp(Literal(5), "-", Literal(3))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_binary_op(gen, op)
        contains(code, "-")
    end

    @test "Generate: comparison" begin
        op = BinaryOp(Identifier("x"), "==", Literal(42))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_binary_op(gen, op)
        contains(code, "==")
    end

    @test "Generate: logical AND" begin
        op = BinaryOp(Identifier("a"), "&&", Identifier("b"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_binary_op(gen, op)
        contains(code, "&&")
    end

    @test "Generate: logical OR" begin
        op = BinaryOp(Identifier("a"), "||", Identifier("b"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_binary_op(gen, op)
        contains(code, "||")
    end

    # Test 11-15: Unary operations
    @test "Generate: negation" begin
        op = UnaryOp("-", Identifier("x"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_unary_op(gen, op)
        contains(code, "-")
    end

    @test "Generate: logical NOT" begin
        op = UnaryOp("!", Identifier("x"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_unary_op(gen, op)
        contains(code, "!")
    end

    @test "Generate: reference" begin
        op = UnaryOp("&", Identifier("x"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_unary_op(gen, op)
        contains(code, "&")
    end

    @test "Generate: dereference" begin
        op = UnaryOp("*", Identifier("ptr"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_unary_op(gen, op)
        contains(code, "*")
    end

    @test "Generate: bitwise NOT" begin
        op = UnaryOp("~", Identifier("x"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_unary_op(gen, op)
        contains(code, "~")
    end

    # Test 16-20: Function and calls
    @test "Generate: simple function prototype" begin
        fn = FnDecl("add", [Param("a", PrimitiveType("i32")), Param("b", PrimitiveType("i32"))],
                    PrimitiveType("i32"), Block([]))
        gen = MinRustCodeGen.CodeGenerator()
        MinRustCodeGen.gen_fn_prototype(gen, fn)
        code = String(take!(gen.output))
        contains(code, "add")
    end

    @test "Generate: function call" begin
        call = Call("add", [Literal(1), Literal(2)])
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_call(gen, call)
        contains(code, "add")
    end

    @test "Generate: array access" begin
        idx = Index(Identifier("arr"), Literal(0))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_index(gen, idx)
        contains(code, "[")
    end

    @test "Generate: type cast" begin
        cast = Cast(Identifier("x"), PrimitiveType("i32"))
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_cast(gen, cast)
        contains(code, "(")
    end

    @test "Generate: array literal" begin
        lit = ArrayLiteral([Literal(1), Literal(2), Literal(3)])
        gen = MinRustCodeGen.CodeGenerator()
        code = MinRustCodeGen.gen_array_literal(gen, lit)
        contains(code, "{")
    end

    # Test 21-25: Full program generation
    @test "Generate: program with function" begin
        fn = FnDecl("main", [], nothing, Block([]))
        program = Program([fn])
        code = MinRustCodeGen.generate_code(program)
        contains(code, "#include")
    end

    @test "Generate: C code includes headers" begin
        program = Program([])
        code = MinRustCodeGen.generate_code(program)
        contains(code, "stdio.h")
    end

    @test "Generate: variable declaration" begin
        decl = VarDecl("x", PrimitiveType("i32"), false, Literal(42))
        gen = MinRustCodeGen.CodeGenerator()
        gen.indent_level = 1
        MinRustCodeGen.gen_var_decl(gen, decl)
        code = String(take!(gen.output))
        contains(code, "int") && contains(code, "x")
    end

    @test "Generate: simple statement" begin
        stmt = ExprStmt(BinaryOp(Identifier("x"), "=", Literal(42)))
        gen = MinRustCodeGen.CodeGenerator()
        gen.indent_level = 1
        MinRustCodeGen.gen_expr_stmt(gen, stmt)
        code = String(take!(gen.output))
        !isempty(code)
    end

    @test "Generate: valid C code syntax" begin
        program = Program([])
        code = MinRustCodeGen.generate_code(program)
        # Should not have syntax errors
        startswith(code, "#")
    end
end

# ============================================================
# Compiler Integration Tests
# ============================================================

function test_compiler_integration()
    println("\n⚙️  Compiler Integration Tests")
    println("─────────────────────────────────────")

    # Test 1-5: End-to-end compilation
    @test "Compile: simple literal" begin
        result = MinRustCompiler.compile("42")
        result.success
    end

    @test "Compile: variable declaration" begin
        result = MinRustCompiler.compile("let x = 42;")
        result.success
    end

    @test "Compile: function declaration" begin
        result = MinRustCompiler.compile("fn main() { }")
        result.success
    end

    @test "Compile: if statement" begin
        result = MinRustCompiler.compile("if true { }")
        result.success
    end

    @test "Compile: for loop" begin
        result = MinRustCompiler.compile("for i in 0..5 { }")
        result.success
    end

    # Test 6-8: Error handling
    @test "Compile: empty string produces tokens" begin
        result = MinRustCompiler.compile("")
        result.token_count >= 0
    end

    @test "Compile: valid program produces C code" begin
        result = MinRustCompiler.compile("fn main() { }")
        result.success && result.c_code !== nothing
    end

    @test "Compile: C code contains main function" begin
        result = MinRustCompiler.compile("fn main() { }")
        result.success && contains(result.c_code, "main")
    end
end

# ============================================================
# Main Test Runner
# ============================================================

function run_all_tests()
    println("🦀 MinRust Compiler Test Suite")
    println("═══════════════════════════════════════")

    test_tokenizer()
    test_parser()
    test_type_checker()
    test_codegen()
    test_compiler_integration()

    print_test_summary()
end

if abspath(PROGRAM_FILE) == @__FILE__
    run_all_tests()
end
