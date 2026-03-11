# 🗺️ MinRust 구현 로드맵

**프로젝트**: MinRust
**문서**: 8-Phase 구현 상세 계획
**작성일**: 2026-03-11 UTC+9

---

## 📅 전체 일정

```
Phase 1: 설계 & 분석 (2일)
   ├─ RUST_SUBSET_DESIGN.md
   ├─ COMPILER_ARCHITECTURE.md
   └─ IMPLEMENTATION_ROADMAP.md (이 문서)

Phase 2: 컴파일러 구현 (5일)
   ├─ Tokenizer (500줄)
   ├─ Parser (1,000줄)
   ├─ AST (400줄)
   ├─ Type Checker (1,000줄)
   ├─ CodeGen (800줄)
   ├─ Compiler (300줄)
   └─ Tests (1,200줄)

Phase 3: 자체호스팅 검증 (2일)
   ├─ Simple Tokenizer (Rust)
   ├─ Bootstrap Verification
   └─ Self-hosting Test

Phase 4: 표준 라이브러리 (3일)
   ├─ I/O Functions (8개)
   ├─ String Functions (12개)
   ├─ Array/Vector Functions (10개)
   ├─ Math Functions (8개)
   ├─ File I/O (6개)
   └─ Tests (60개)

Phase 5: Rust 자체호스팅 컴파일러 (4일)
   ├─ Tokenizer (Rust)
   ├─ Parser (Rust)
   ├─ Type Checker (Rust)
   ├─ CodeGen (Rust)
   └─ Tests (40개)

Phase 6: 고급 최적화 (2일)
   ├─ Constant Propagation
   ├─ Dead Code Elimination
   ├─ Loop Optimization
   ├─ Function Inlining
   └─ Tests (30개)

Phase 7: 고급 기능 (3일)
   ├─ Generics (기초)
   ├─ Traits (기초)
   ├─ Pattern Matching (강화)
   ├─ Modules (기초)
   ├─ Macros (기초)
   └─ Tests (25개)

Phase 8: 생태계 구축 (2일)
   ├─ VSCode Integration
   ├─ Online Playground
   ├─ Cargo System
   └─ Documentation

Total: ~24-28일 (동시 병렬 가능)
```

---

## 🔧 Phase 2: 컴파일러 구현 (상세)

### 2-1: Tokenizer (500줄)

**파일**: `src/tokenizer.jl`

**구현 순서**:

```julia
1. Token 타입 정의 (enum)
   TokenType = enum(
     KEYWORD, IDENT, INT, FLOAT, STRING, CHAR, BOOLEAN,
     LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET,
     COMMA, SEMICOLON, COLON, COLONCOLON, DOT, ARROW, FAT_ARROW,
     PLUS, MINUS, STAR, SLASH, PERCENT, EQ, NE, LT, GT, LE, GE,
     ASSIGN, PLUS_ASSIGN, MINUS_ASSIGN, STAR_ASSIGN, SLASH_ASSIGN,
     AND, OR, NOT, BIT_AND, BIT_OR, BIT_XOR, BIT_NOT,
     LSHIFT, RSHIFT, AMPERSAND, PIPE, CARET, TILDE,
     QUESTION, BACKTICK, HASH, AT, DOLLAR, NEWLINE, EOF
   )

2. Token 구조체
   struct Token
     type::TokenType
     value::Union{String, Int, Float}
     line::Int
     column::Int
     start::Int
     end::Int
   end

3. Tokenizer 구조체
   struct Tokenizer
     input::String
     pos::Int
     line::Int
     column::Int
     tokens::Vector{Token}
   end

4. 주요 함수
   tokenize(input::String) → Vector{Token}
   scan_token() → Token
   peek_char() → Char
   advance() → Char
   is_keyword(word) → Bool
   scan_number() → Token
   scan_string() → Token
   scan_identifier() → Token

5. 키워드 맵
   KEYWORDS = Dict(
     "fn" => KEYWORD_FN,
     "let" => KEYWORD_LET,
     "const" => KEYWORD_CONST,
     "mut" => KEYWORD_MUT,
     "if" => KEYWORD_IF,
     "else" => KEYWORD_ELSE,
     "for" => KEYWORD_FOR,
     "while" => KEYWORD_WHILE,
     "loop" => KEYWORD_LOOP,
     "match" => KEYWORD_MATCH,
     "struct" => KEYWORD_STRUCT,
     "impl" => KEYWORD_IMPL,
     "trait" => KEYWORD_TRAIT,
     ... (총 25+개)
   )
```

**테스트** (40개):
```
✓ Basic tokens (keywords, identifiers, numbers)
✓ String literals (single, multi-line, escape)
✓ Operators (arithmetic, comparison, logical, bitwise)
✓ Comments (line comment //, block comment /* */)
✓ Location tracking (line, column numbers)
✓ Complex expressions
```

**예시 테스트**:
```julia
test("basic tokens") do
  tokens = tokenize("let x = 42;")
  @test tokens[1].type == KEYWORD_LET
  @test tokens[2].value == "x"
  @test tokens[4].value == 42
end
```

---

### 2-2: Parser (1,000줄)

**파일**: `src/parser.jl`

**구현 순서**:

```julia
1. AST 노드 타입 정의
   abstract type ASTNode end
   struct Program <: ASTNode
     items::Vector{ASTNode}
   end
   struct FnDecl <: ASTNode
     name::String
     params::Vector{Param}
     return_type::Union{Type, Nothing}
     body::Block
   end
   struct VarDecl <: ASTNode
     name::String
     type_annotation::Union{Type, Nothing}
     init::Union{Expr, Nothing}
   end
   struct IfStmt <: ASTNode
     condition::Expr
     then_block::Block
     else_block::Union{Block, Nothing}
   end
   ... (50+ 노드 타입)

2. Parser 구조체
   struct Parser
     tokens::Vector{Token}
     pos::Int
     current_token::Token
   end

3. 주요 함수 (Recursive Descent)
   parseProgram()
   parseDeclaration()
   parseFnDecl()
   parseStructDecl()
   parseImplBlock()
   parseStatement()
   parseVarDecl()
   parseConstDecl()
   parseIfStmt()
   parseForStmt()
   parseWhileStmt()
   parseLoopStmt()
   parseMatchStmt()
   parseExpression()
   parseAssignment()
   parseLogicalOr()
   parseLogicalAnd()
   parseEquality()
   parseComparison()
   parseAdditive()
   parseMultiplicative()
   parseUnary()
   parsePostfix()
   parsePrimary()

4. 헬퍼 함수
   peek() → Token
   advance() → Token
   expect(type) → Token
   match(types...) → Bool
   synchronize() → void (에러 복구)
```

**문법 규칙** (EBNF 형식):

```ebnf
program = (fn_decl | struct_decl | impl_block)*

fn_decl = "fn" IDENT "(" param_list? ")" ["->" type] block

param_list = param ("," param)*
param = IDENT ":" type

struct_decl = "struct" IDENT "{" field_list? "}"
field_list = field ("," field)*
field = IDENT ":" type

impl_block = "impl" [type_param] type "{" (fn_decl)* "}"

statement = var_decl
          | const_decl
          | if_stmt
          | for_stmt
          | while_stmt
          | loop_stmt
          | match_stmt
          | return_stmt
          | break_stmt
          | continue_stmt
          | expr_stmt

var_decl = "let" "mut"? IDENT [":" type] "=" expression ";"
const_decl = "const" IDENT ":" type "=" expression ";"

if_stmt = "if" expression block ("else" block)?
for_stmt = "for" IDENT "in" expression block
while_stmt = "while" expression block
loop_stmt = "loop" block
match_stmt = "match" expression "{" match_arm* "}"
match_arm = pattern "=>" expression ","

expression = assignment
assignment = logical_or ("=" logical_or)*
logical_or = logical_and ("||" logical_and)*
logical_and = equality ("&&" equality)*
equality = comparison (("==" | "!=") comparison)*
comparison = additive (("<" | ">" | "<=" | ">=") additive)*
additive = multiplicative (("+" | "-") multiplicative)*
multiplicative = unary (("*" | "/" | "%") unary)*
unary = ("!" | "-" | "&" | "*")* postfix
postfix = primary (call | index | field_access)*
primary = INT | FLOAT | STRING | CHAR | BOOL
        | IDENT
        | "(" expression ")"
        | "[" expression_list? "]"
        | "vec!" "[" expression_list "]"
        | struct_name "{" field_init_list "}"
```

**테스트** (30개):
```
✓ Simple expressions (42, x, "hello")
✓ Binary operations (+, -, *, /, %, ==, !=, <, >, etc.)
✓ Variable declarations (let, const, mut)
✓ Function declarations
✓ Function calls
✓ Control flow (if/else, for, while, loop, match)
✓ Array/Vector literals
✓ Struct definitions and instantiation
✓ Operator precedence
✓ Complex nested expressions
```

---

### 2-3: AST (400줄)

**파일**: `src/ast.jl`

```julia
# 기본 AST 노드
abstract type ASTNode end

# 프로그램
struct Program <: ASTNode
  items::Vector{ASTNode}
end

# 선언 (Declarations)
struct FnDecl <: ASTNode
  name::String
  params::Vector{Param}
  return_type::Union{Type, Nothing}
  body::Block
end

struct Param
  name::String
  type_annotation::Type
end

struct StructDecl <: ASTNode
  name::String
  fields::Vector{StructField}
end

struct StructField
  name::String
  type_annotation::Type
end

# 타입 (Types)
abstract type RustType end

struct PrimitiveType <: RustType
  name::String  # "i32", "f64", "bool", "String", etc.
end

struct ReferenceType <: RustType
  inner::RustType
  mutable::Bool  # true for &mut, false for &
end

struct ArrayType <: RustType
  element_type::RustType
  size::Union{Int, Nothing}  # Nothing for slices
end

# 블록
struct Block <: ASTNode
  statements::Vector{ASTNode}
end

# 문 (Statements)
struct VarDecl <: ASTNode
  name::String
  type_annotation::Union{RustType, Nothing}
  is_mut::Bool
  init::Union{Expr, Nothing}
end

struct ExprStmt <: ASTNode
  expr::Expr
end

struct IfStmt <: ASTNode
  condition::Expr
  then_block::Block
  else_block::Union{Block, Nothing}
end

# 식 (Expressions)
abstract type Expr <: ASTNode end

struct Literal <: Expr
  value::Union{Int, Float, String, Char, Bool}
  type::RustType
end

struct Identifier <: Expr
  name::String
end

struct BinaryOp <: Expr
  left::Expr
  op::String  # "+", "-", "*", "/", "==", "!=", "<", etc.
  right::Expr
end

struct UnaryOp <: Expr
  op::String  # "-", "!", "&", "*"
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
```

---

### 2-4: Type Checker (1,000줄)

**파일**: `src/type_checker.jl`

**주요 함수**:

```julia
struct TypeChecker
  symbol_table::Dict{String, Any}
  current_scope::Vector{Dict{String, Any}}
  errors::Vector{String}
end

# 메인 진입점
function check_types(ast::Program) :: TypedProgram
  checker = TypeChecker()
  result = check_program(checker, ast)
  if !isempty(checker.errors)
    throw(TypeCheckError(checker.errors))
  end
  return result
end

# 스코프 관리
function push_scope(checker::TypeChecker)
  push!(checker.current_scope, Dict())
end

function pop_scope(checker::TypeChecker)
  pop!(checker.current_scope)
end

# 심볼 등록
function register_symbol(checker, name::String, type::RustType)
  checker.current_scope[end][name] = type
end

# 심볼 조회
function lookup_symbol(checker, name::String) :: RustType
  for scope in reverse(checker.current_scope)
    if haskey(scope, name)
      return scope[name]
    end
  end
  error("Undefined variable: $name")
end

# 타입 호환성 검사
function is_compatible(from::RustType, to::RustType) :: Bool
  # 같은 타입
  if from == to
    return true
  end
  # 정수 타입 호환성 (제한적)
  if isa(from, PrimitiveType) && isa(to, PrimitiveType)
    # i32와 i64는 호환되지 않음 (명시적 캐스트 필요)
  end
  return false
end

# 식 타입 검사
function infer_type(checker, expr::Expr) :: RustType
  if isa(expr, Literal)
    return expr.type
  elseif isa(expr, Identifier)
    return lookup_symbol(checker, expr.name)
  elseif isa(expr, BinaryOp)
    left_type = infer_type(checker, expr.left)
    right_type = infer_type(checker, expr.right)
    return check_binary_op(left_type, right_type, expr.op)
  elseif isa(expr, Call)
    return check_function_call(checker, expr)
  # ... 더 많은 식 타입
  end
end

# 이진 연산 타입 검사
function check_binary_op(left::RustType, right::RustType, op::String) :: RustType
  if op in ["+", "-", "*", "/", "%"]
    # 산술 연산: 두 피연산자가 같은 정수/실수 타입이어야 함
    if left == right && isa(left, PrimitiveType)
      return left
    end
  elseif op in ["==", "!=", "<", ">", "<=", ">="]
    # 비교 연산: bool 반환
    if left == right
      return PrimitiveType("bool")
    end
  # ... 더 많은 연산
  end
  error("Type mismatch in binary operation")
end

# 함수 호출 검사
function check_function_call(checker, call::Call) :: RustType
  # 함수 서명 조회
  # 인자 개수/타입 검사
  # 반환 타입 반환
end
```

**테스트** (35개):
```
✓ Variable type inference
✓ Function signature validation
✓ Type compatibility checking
✓ Function argument type checking
✓ Return type validation
✓ Scope validation
✓ Reference validation
✓ Array/Vector type checking
✓ Error messages
```

---

### 2-5: Code Generation (800줄)

**파일**: `src/codegen.jl`

```julia
struct CodeGenerator
  output::IOBuffer
  var_counter::Int
  label_counter::Int
  current_function::Union{String, Nothing}
end

# LLVM IR 생성
function generate_code(ast::TypedProgram) :: String
  gen = CodeGenerator()

  # 헤더
  println(gen.output, "; Generated LLVM IR")

  # 선언 처리
  for item in ast.items
    generate_item(gen, item)
  end

  return String(take!(gen.output))
end

# 함수 코드 생성
function generate_function(gen, func::FnDecl)
  gen.current_function = func.name

  # 함수 서명
  return_type = llvm_type(func.return_type)
  params = join([llvm_type(p.type_annotation) * " %" * p.name for p in func.params], ", ")
  println(gen.output, "define $return_type @$(func.name)($params) {")

  # 함수 본체
  for stmt in func.body.statements
    generate_statement(gen, stmt)
  end

  println(gen.output, "}")
end

# 변수 선언
function generate_var_decl(gen, decl::VarDecl)
  llvm_type_str = llvm_type(decl.type_annotation)
  var_ref = "%$(decl.name)"

  # 스택 할당
  println(gen.output, "  $var_ref = alloca $llvm_type_str")

  # 초기값 할당
  if decl.init !== nothing
    init_val = generate_expression(gen, decl.init)
    println(gen.output, "  store $llvm_type_str $init_val, $llvm_type_str* $var_ref")
  end
end

# 식 코드 생성
function generate_expression(gen, expr::Expr) :: String
  if isa(expr, Literal)
    return generate_literal(gen, expr)
  elseif isa(expr, Identifier)
    return generate_load(gen, expr.name)
  elseif isa(expr, BinaryOp)
    return generate_binary_op(gen, expr)
  elseif isa(expr, Call)
    return generate_call(gen, expr)
  end
end

# 리터럴
function generate_literal(gen, lit::Literal) :: String
  if isa(lit.value, Int)
    return "$(lit.value)"
  elseif isa(lit.value, Float)
    return "$(lit.value)"
  elseif isa(lit.value, String)
    return generate_string_literal(gen, lit.value)
  end
end

# 로드 (메모리에서 읽기)
function generate_load(gen, var_name::String) :: String
  var_ref = gen_temp_var(gen)
  println(gen.output, "  $var_ref = load i32, i32* %$var_name")
  return var_ref
end

# 이진 연산
function generate_binary_op(gen, op::BinaryOp) :: String
  left = generate_expression(gen, op.left)
  right = generate_expression(gen, op.right)
  result = gen_temp_var(gen)

  llvm_op = rustop_to_llvmop(op.op)
  println(gen.output, "  $result = $llvm_op i32 $left, $right")

  return result
end

# 임시 변수 생성
function gen_temp_var(gen)
  gen.var_counter += 1
  return "%tmp$(gen.var_counter)"
end

# 레이블 생성
function gen_label(gen, prefix="label")
  gen.label_counter += 1
  return prefix * string(gen.label_counter)
end
```

**C 코드 생성** (대안):
```julia
function generate_c_code(ast::TypedProgram) :: String
  c_code = "#include <stdio.h>\n"
  c_code *= "#include <string.h>\n\n"

  for item in ast.items
    c_code *= generate_c_item(item)
  end

  return c_code
end
```

---

### 2-6: Compiler Integration (300줄)

**파일**: `src/compiler.jl`

```julia
struct CompilationResult
  success::Bool
  ir_code::Union{String, Nothing}
  c_code::Union{String, Nothing}
  errors::Vector{String}
end

function compile(input::String) :: CompilationResult
  # Step 1: Tokenize
  tokens = tokenize(input)

  # Step 2: Parse
  ast = parse(tokens)

  # Step 3: Type Check
  typed_ast = check_types(ast)

  # Step 4: Generate IR
  ir_code = generate_ir(typed_ast)

  # Step 5: Optimize (Phase 6+)
  optimized_ir = optimize(ir_code)

  # Step 6: Generate C code
  c_code = generate_c_from_ir(optimized_ir)

  return CompilationResult(
    success = true,
    ir_code = ir_code,
    c_code = c_code,
    errors = []
  )
end
```

**테스트** (25개):
```
✓ End-to-end compilation
✓ Error propagation
✓ IR generation
✓ C code generation
✓ Complex programs
```

---

## 📊 Phase 2 최종 통계

```
파일: 6개 (tokenizer, parser, ast, type_checker, codegen, compiler)
코드: 4,400줄 (구현)
테스트: 1,200줄 (130개 테스트 케이스)
문서: 생성됨 (자동)
```

---

## 🔄 Phase 3: 자체호스팅 검증

**목표**: MinRust 컴파일러를 MinRust로 부트스트랩

**구현**:
1. `src/simple_tokenizer.rs` - Tokenizer의 단순 Rust 구현
2. `test/self_hosting_test.jl` - Julia 컴파일러로 검증
3. 부트스트랩 사이클 (Rust로 작성한 코드를 원래 컴파일러로 컴파일)

---

## 🎁 Phase 4: 표준 라이브러리

**40+ 함수 분류**:

| 카테고리 | 함수 | 개수 |
|---------|------|-----|
| I/O | println, print, input, output | 8 |
| String | len, to_upper, to_lower, trim, split, replace | 12 |
| Array/Vec | len, push, pop, map, filter | 10 |
| Math | abs, max, min, pow, sqrt, gcd | 8 |
| File | open, read, write, close | 6 |
| Misc | type, range, clone | - |

---

## 🚀 Phase 5: Rust 자체호스팅

**목표**: 컴파일러를 완전히 Rust로 재작성

**파일**:
- `src/lib.rs` - 핵심 라이브러리
- `src/tokenizer.rs` (300줄)
- `src/parser.rs` (400줄)
- `src/type_checker.rs` (300줄)
- `src/codegen.rs` (200줄)

---

## ⚡ Phase 6: 최적화

**5가지 최적화 기법**:

1. 상수 전파
2. 데드 코드 제거
3. 루프 최적화
4. 함수 인라인화
5. 타입 기반 최적화

---

## 🎯 Phase 7: 고급 기능

1. 제너릭 (기초)
2. 트레이트 (기초)
3. 패턴 매칭 (강화)
4. 모듈 시스템 (기초)
5. 매크로 (기초)

---

## 🌍 Phase 8: 생태계 구축

1. **VSCode TextMate 문법** (80줄)
2. **온라인 Playground** (300줄)
3. **Cargo.toml 파서** (150줄)
4. **문서 및 예제** (100줄)

---

**다음**: Phase 2 컴파일러 구현 시작
