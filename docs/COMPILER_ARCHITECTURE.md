# 🏗️ MinRust 컴파일러 아키텍처

**프로젝트**: MinRust
**문서**: 컴파일러 6단계 파이프라인
**작성일**: 2026-03-11 UTC+9

---

## 📋 전체 아키텍처 다이어그램

```
┌─────────────────────────────────────────────────────────────────┐
│                     MinRust Compiler Pipeline                    │
└─────────────────────────────────────────────────────────────────┘

[Input: main.rs]
        ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 1: TOKENIZATION (Lexer)                                    │
│ ─────────────────────────────────────────────────────────────── │
│ Task: Source code → Tokens                                       │
│ Input: "let x: i32 = 42;"                                        │
│ Output: [LET, IDENT(x), COLON, IDENT(i32), ASSIGN, INT(42),     │
│          SEMICOLON]                                              │
└─────────────────────────────────────────────────────────────────┘
        ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 2: PARSING (Syntax Analysis)                               │
│ ─────────────────────────────────────────────────────────────── │
│ Task: Tokens → Abstract Syntax Tree (AST)                       │
│ Algorithm: Recursive Descent Parser                              │
│ Output: AST nodes (VarDecl, FnDecl, Block, If, For, etc.)      │
│                                                                  │
│ Grammar Example:                                                 │
│   program := (fn_decl | var_decl | struct_decl)*                │
│   fn_decl := "fn" IDENT "(" params ")" ["->" type] block        │
│   stmt := var_decl | expr_stmt | if_stmt | for_stmt | ...       │
│   expr := assignment | logical_or                               │
└─────────────────────────────────────────────────────────────────┘
        ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 3: TYPE CHECKING (Semantic Analysis)                       │
│ ─────────────────────────────────────────────────────────────── │
│ Task: AST + Type Validation                                      │
│ Operations:                                                      │
│   • Type inference (let x = 42 → x: i32)                        │
│   • Type compatibility (i32 != f64)                             │
│   • Function signature validation                               │
│   • Variable scope validation                                   │
│   • Reference validation                                        │
│                                                                  │
│ Output: Typed AST + Symbol Table                                │
└─────────────────────────────────────────────────────────────────┘
        ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 4: IR GENERATION (Intermediate Representation)             │
│ ─────────────────────────────────────────────────────────────── │
│ Task: Typed AST → LLVM IR (Intermediate Code)                   │
│ Operations:                                                      │
│   • Variable allocation (stack/heap)                            │
│   • Expression compilation                                      │
│   • Function calls                                              │
│   • Control flow (branches, loops)                              │
│                                                                  │
│ Example:                                                         │
│   let x: i32 = 42;           →  %x = alloca i32                │
│   x + 10                     →  %tmp = add i32 %x, 10          │
│   if x > 0 { ... }           →  br i1 %cond, label %if.then,   │
│                                 label %if.else                   │
└─────────────────────────────────────────────────────────────────┘
        ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 5: OPTIMIZATION (Optional)                                 │
│ ─────────────────────────────────────────────────────────────── │
│ Task: LLVM IR → Optimized LLVM IR                               │
│ Techniques:                                                      │
│   • Constant Propagation (const 42 → directly use 42)          │
│   • Dead Code Elimination (unused vars/instrs removed)          │
│   • Loop Unrolling (for i in 0..3 → manual unroll)             │
│   • Function Inlining (small functions inlined at call site)    │
│   • Type-Based Optimization (type info for better codegen)      │
│                                                                  │
│ Input/Output: LLVM IR → Optimized LLVM IR                       │
└─────────────────────────────────────────────────────────────────┘
        ↓
┌─────────────────────────────────────────────────────────────────┐
│ Step 6: CODE GENERATION                                         │
│ ─────────────────────────────────────────────────────────────── │
│ Task: LLVM IR → Machine Code / Assembly                         │
│ Options:                                                         │
│   • Direct LLVM backend (if available)                          │
│   • C code generation (our approach)                            │
│   • x86-64 assembly                                             │
│                                                                  │
│ Output: Executable or Object File                               │
└─────────────────────────────────────────────────────────────────┘
        ↓
[Output: executable / main.o / LLVM IR]
```

---

## 🔧 각 단계 상세 설명

### Step 1: Tokenization (Lexical Analysis)

**목표**: 소스 코드 문자열을 의미있는 토큰으로 변환

**토큰 타입** (60+개):

```
[Keywords]
fn, let, const, mut, if, else, for, while, loop, match,
break, continue, return, struct, impl, trait, pub, pub(crate),
use, mod, crate, as, dyn, unsafe, static, async, await, move,
ref, in, where, type, enum, union

[Literals]
Integer: 42, 0xFF, 0b1010
Float: 3.14, 1.0e-5
String: "hello", 'c', "multi\nline"
Boolean: true, false

[Operators]
Arithmetic: +, -, *, /, %
Comparison: ==, !=, <, >, <=, >=
Logical: &&, ||, !
Bitwise: &, |, ^, <<, >>, ~
Assignment: =, +=, -=, *=, /=, %=
Other: ->, =>, :, ::, ., ,, ;

[Punctuation]
( ) { } [ ] < > " ' # @ $ _ ...

[Identifiers]
variable_name, FunctionName, CONSTANT_NAME, type_name
```

**실제 예시**:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Tokens:
// [KEYWORD(fn), IDENT(add), LPAREN, IDENT(a), COLON, IDENT(i32),
//  COMMA, IDENT(b), COLON, IDENT(i32), RPAREN, ARROW, IDENT(i32),
//  LBRACE, IDENT(a), PLUS, IDENT(b), RBRACE]
```

**구현**: `src/tokenizer.jl` (500줄)
- 문자별 스캔
- 키워드/식별자 인식
- 문자열 이스케이프 처리
- 위치 정보 추적 (line, column)

---

### Step 2: Parsing (Syntax Analysis)

**목표**: 토큰 스트림을 구조화된 AST로 변환

**파서 타입**: Recursive Descent Parser

**주요 파싱 함수**:

```
parseProgram()
  ├─ parseFnDecl()         // fn name() { ... }
  ├─ parseStructDecl()     // struct Name { ... }
  └─ parseImplBlock()      // impl Type { ... }

parseStatement()
  ├─ parseVarDecl()        // let x = ...
  ├─ parseConstDecl()      // const X = ...
  ├─ parseIfStmt()         // if ... { ... } else { ... }
  ├─ parseForStmt()        // for x in 0..10 { ... }
  ├─ parseWhileStmt()      // while ... { ... }
  ├─ parseLoopStmt()       // loop { ... }
  ├─ parseMatchStmt()      // match x { ... }
  └─ parseExprStmt()       // expr;

parseExpression()
  ├─ parseAssignment()     // x = y
  ├─ parseLogicalOr()      // a || b
  ├─ parseLogicalAnd()     // a && b
  ├─ parseEquality()       // a == b, a != b
  ├─ parseComparison()     // a < b, a > b, ...
  ├─ parseBitwise()        // a & b, a | b, ...
  ├─ parseAdditive()       // a + b, a - b
  ├─ parseMultiplicative() // a * b, a / b, a % b
  ├─ parseUnary()          // -a, !b, &c, *ptr
  ├─ parsePostfix()        // f(x), a[i], a.field
  └─ parsePrimary()        // 42, x, "str", (expr)
```

**예시: 파싱 과정**

```
Input:  let x: i32 = 42;
Tokens: [LET, IDENT(x), COLON, IDENT(i32), ASSIGN, INT(42), SEMICOLON]

Parsing:
  parseStatement() → parseVarDecl()
    ├─ expect(LET) → consume LET
    ├─ name = expect(IDENT) → "x"
    ├─ type = parseType() → i32 (from COLON, IDENT)
    ├─ expect(ASSIGN) → consume =
    ├─ expr = parseExpression() → Literal(42)
    └─ expect(SEMICOLON) → consume ;

AST:
VarDecl {
  name: "x",
  type: Some(i32),
  init: Some(Literal(42))
}
```

**AST 노드 종류** (50+):

```
Program
├─ FnDecl
├─ StructDecl
├─ ImplBlock
├─ Statements
│  ├─ VarDecl
│  ├─ ConstDecl
│  ├─ ExprStmt
│  ├─ IfStmt
│  ├─ ForStmt
│  ├─ WhileStmt
│  ├─ LoopStmt
│  ├─ MatchStmt
│  ├─ ReturnStmt
│  └─ BreakStmt
└─ Expressions
   ├─ Literal (Int, Float, Bool, String)
   ├─ Identifier
   ├─ BinaryOp (Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Gt, Le, Ge, And, Or, BitwiseAnd, BitwiseOr, BitwiseXor, Shl, Shr)
   ├─ UnaryOp (Neg, Not, BitwiseNot, Deref, Ref, RefMut)
   ├─ Call (function call)
   ├─ MethodCall (object.method())
   ├─ FieldAccess (object.field)
   ├─ Index (array[i])
   ├─ Cast (expr as Type)
   ├─ Range (0..10)
   ├─ IfExpr (if x { y } else { z })
   ├─ MatchExpr
   ├─ ArrayLiteral ([1, 2, 3])
   ├─ VecMacro (vec![1, 2, 3])
   ├─ StructLiteral (Point { x: 1, y: 2 })
   └─ Closure (|x| x + 1)
```

**구현**: `src/parser.jl` (1,000줄)
- 연산자 우선순위 처리
- 에러 복구
- 위치 정보 유지

---

### Step 3: Type Checking (Semantic Analysis)

**목표**: 타입 안정성 및 의미 검증

**주요 작업**:

```
1. Symbol Table Construction
   ├─ 함수 정의 수집
   ├─ 변수/상수 범위 관리
   ├─ 구조체 정의 수집
   └─ 기본 타입 등록

2. Type Inference
   ├─ let x = 42 → i32
   ├─ let y = 3.14 → f64
   ├─ let z = &x → &i32
   └─ 암시적 타입 추론

3. Type Compatibility Checking
   ├─ x: i32 = 3.14? → Error
   ├─ let y = x + 10 → OK (both i32)
   ├─ 함수 인자 타입 검증
   └─ 반환 타입 검증

4. Reference Validation
   ├─ &x → Reference 생성
   ├─ *ptr → Dereference
   ├─ 참조 수명 (기초, 구체적 라이프타임 없음)
   └─ Mutable reference 규칙

5. Function Validation
   ├─ 함수 인자 개수 확인
   ├─ 인자 타입 확인
   ├─ 반환 타입 확인
   └─ 모든 경로에서 반환 확인

6. Scope Validation
   ├─ 변수 중복 선언 확인
   ├─ 사용 전 정의 확인
   └─ 범위를 벗어난 사용 확인
```

**예시: 타입 검증**

```rust
Input AST:
fn add(a: i32, b: i32) -> i32 {
    a + b
}

Type Checking:
1. Register function "add" with signature (i32, i32) → i32
2. In function body:
   - Variable 'a' has type i32
   - Variable 'b' has type i32
   - Expression (a + b):
     * a is i32 ✓
     * b is i32 ✓
     * i32 + i32 = i32 ✓
   - Return type is i32 ✓
3. Result: ✓ Type-checked successfully

Another example (with error):
fn test() -> i32 {
    let x: i32 = 42;
    x + 3.14  // Error: i32 + f64
}

Type Checking:
- x is i32
- 3.14 is f64
- i32 + f64 → Type Mismatch Error!
```

**구현**: `src/type_checker.jl` (1,000줄)
- 심볼 테이블 관리
- 타입 추론 엔진
- 에러 메시지 생성

---

### Step 4: IR Generation

**목표**: Typed AST를 저수준 LLVM IR로 변환

**LLVM IR 예시**:

```llvm
; Rust code: let x: i32 = 42; println!("{}", x);
; Generated LLVM IR:

@format_str = private constant [4 x i8] c"%d\0A\00"

define i32 @main() {
  %x = alloca i32
  store i32 42, i32* %x
  %val = load i32, i32* %x
  %format = load [4 x i8]*, [4 x i8]** @format_str
  %call = call i32 (i8*, ...) @printf(i8* %format, i32 %val)
  ret i32 0
}

declare i32 @printf(i8*, ...)
```

**IR 생성 규칙**:

```
Variable Declaration:
  let x: i32 = 42;
  → %x = alloca i32           (스택 할당)
  → store i32 42, i32* %x     (값 저장)

Variable Use:
  x + 10
  → %tmp = load i32, i32* %x  (값 로드)
  → %result = add i32 %tmp, 10

Function Call:
  add(x, y)
  → %call = call i32 @add(i32 %x_val, i32 %y_val)

Control Flow (if):
  if x > 0 { print("pos") } else { print("neg") }
  → %cond = icmp sgt i32 %x_val, 0
  → br i1 %cond, label %if.then, label %if.else
  if.then:
    ...
    br label %if.end
  if.else:
    ...
    br label %if.end
  if.end:
    ...

Loop (for):
  for i in 0..10 { ... }
  → br label %for.cond
  for.cond:
    %cond = icmp slt i32 %i, 10
    br i1 %cond, label %for.body, label %for.end
  for.body:
    ... (loop body)
    %i.next = add i32 %i, 1
    br label %for.cond
  for.end:
    ...
```

**구현**: `src/codegen.jl` (800줄)
- 스택 프레임 관리
- 레지스터 할당 (간단한 형태)
- 함수 호출 규약
- 제어 흐름 생성

---

### Step 5: Optimization

**목표**: 생성된 IR 최적화

**최적화 기법**:

```
1. Constant Propagation
   Input:  const X = 42; let y = X + 10;
   Output: let y = 52;

2. Dead Code Elimination
   Input:  let x = 100; (x never used)
   Output: (removed)

3. Loop Unrolling
   Input:  for i in 0..4 { sum += arr[i]; }
   Output: sum += arr[0];
           sum += arr[1];
           sum += arr[2];
           sum += arr[3];

4. Function Inlining
   Input:  fn add(a, b) { a + b }
           result = add(1, 2);
   Output: result = 1 + 2;

5. Type-Based Optimization
   Input:  &x (reference to x)
   Output: Direct access (no indirection if possible)
```

**구현**: `src/optimizer.jl` (400줄)

---

### Step 6: Code Generation

**목표**: IR을 실행 가능한 코드로 변환

**접근 방식** (세 가지 선택):

```
방식 1: Direct LLVM Backend
- LLVM C API 사용
- 최고 성능
- 복잡함

방식 2: C Code Generation (우리의 접근)
- LLVM IR → C 코드로 번역
- gcc/clang으로 컴파일
- 간단하고 이식성 좋음
- 성능 적당함

방식 3: x86-64 Assembly
- LLVM IR → x86-64 asm
- 최고 성능
- 복잡하고 platform-specific
```

**우리는 방식 2를 선택**:

```rust
Input LLVM IR:
define i32 @main() {
  %x = alloca i32
  store i32 42, i32* %x
  %val = load i32, i32* %x
  ret i32 %val
}

↓ (변환)

Output C code:
#include <stdio.h>

int main() {
  int x = 42;
  int val = x;
  return val;
}

↓ (gcc 컴파일)

Output: executable
```

**구현**: `src/codegen.jl` 의 C 생성 부분 (500줄)

---

## 📊 구현 단계별 라인 수

| 구성요소 | 파일 | 줄 수 | 역할 |
|---------|------|-------|------|
| **Tokenizer** | src/tokenizer.jl | 500 | 어휘 분석 |
| **Parser** | src/parser.jl | 1,000 | 구문 분석 |
| **AST** | src/ast.jl | 400 | 추상 구문 트리 |
| **Type Checker** | src/type_checker.jl | 1,000 | 의미 분석 |
| **CodeGen** | src/codegen.jl | 800 | 코드 생성 |
| **Compiler** | src/compiler.jl | 300 | 통합 |
| **Optimizer** | src/optimizer.jl | 400 | 최적화 (Phase 6) |
| **テスト** | test/*.jl | 1,200 | 130+ 테스트 케이스 |

---

## 🔍 데이터 흐름 예시

**예제 코드**:

```rust
fn main() {
    let x = 42;
    if x > 40 {
        println!("{}", x);
    }
}
```

**단계별 변환**:

```
[1] Tokenization:
    [FN, IDENT(main), LPAREN, RPAREN, LBRACE,
     LET, IDENT(x), ASSIGN, INT(42), SEMICOLON,
     IF, IDENT(x), GT, INT(40), LBRACE,
     PRINTLN_MACRO, ..., RBRACE, RBRACE]

[2] Parsing:
    Program {
      items: [
        FnDecl {
          name: "main",
          params: [],
          return_type: None,
          body: Block {
            stmts: [
              VarDecl {
                name: "x",
                type: None (infer i32),
                init: Literal(42)
              },
              IfStmt {
                cond: BinaryOp(
                  Identifier("x"),
                  Gt,
                  Literal(40)
                ),
                then_block: [...],
                else_block: None
              }
            ]
          }
        }
      ]
    }

[3] Type Checking:
    Symbol table:
    - main() → void
    - x → i32

    Type validation:
    - x > 40: i32 > i32 ✓
    - x in println: i32 ✓

[4] IR Generation:
    define void @main() {
      %x = alloca i32
      store i32 42, i32* %x
      %x_val = load i32, i32* %x
      %cond = icmp sgt i32 %x_val, 40
      br i1 %cond, label %if.then, label %if.end
    if.then:
      %format = load i8*, i8** @format_str
      %x_val2 = load i32, i32* %x
      call i32 (i8*, ...) @printf(i8* %format, i32 %x_val2)
      br label %if.end
    if.end:
      ret void
    }

[5] Optimization:
    (상수 전파: 42는 컴파일타임에 알려짐)
    (루프 없으므로 루프 최적화 불필요)

[6] C Code Generation:
    #include <stdio.h>
    void main() {
      int x = 42;
      if (x > 40) {
        printf("%d\n", x);
      }
    }

[7] C Compilation:
    gcc -o main main.c
    → executable: ./main
```

---

**다음 문서**: `IMPLEMENTATION_ROADMAP.md` (구현 로드맵)
