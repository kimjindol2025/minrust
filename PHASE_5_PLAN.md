# Phase 5: Rust 자체호스팅 컴파일러 구현 계획

**목표**: MinRust 컴파일러를 Rust로 완전히 재작성하여 자체호스팅 컴파일러 달성

**예상 코드량**: 4,000+ 줄 (Rust 구현)
**예상 테스트**: 50+ 통합 테스트
**마일스톤**: 컴파일러가 자신을 컴파일 가능

---

## 📋 프로젝트 구조 (현재 상태)

```
src_rust/
├── Cargo.toml           ✅ 기본 패키지 설정
├── src/
│   ├── lib.rs           ✅ 라이브러리 진입점 (43줄)
│   ├── main.rs          ✅ CLI 진입점 (105줄)
│   ├── tokenizer.rs     ✅ 완성 (670줄, 5 테스트)
│   ├── ast.rs           ✅ AST 정의 (240줄, 2 테스트)
│   ├── parser.rs        🔄 스켈레톤 (90줄, 2 테스트)
│   ├── type_checker.rs  🔄 스켈레톤 (95줄, 2 테스트)
│   ├── codegen.rs       🔄 스켈레톤 (135줄, 3 테스트)
│   └── compiler.rs      ✅ 파이프라인 (110줄, 3 테스트)
└── tests/
    └── integration_tests.rs (계획중)
```

**현재 상태**:
- ✅ 프로젝트 구조 완성
- ✅ Tokenizer 완전 구현 (670줄)
- ✅ CLI 인터페이스 완성
- 🔄 Parser, TypeChecker, CodeGen 스켈레톤 완성
- ⏳ Parser 상세 구현 대기
- ⏳ TypeChecker 상세 구현 대기
- ⏳ CodeGen 상세 구현 대기

---

## 🎯 Phase 5 상세 구현 계획

### Phase 5.1: 토크나이저 완성 ✅
**상태**: 완료
**구현 사항**:
- ✅ 60+ 토큰 타입 지원
- ✅ 모든 키워드 인식 (34개)
- ✅ 주석 처리 (라인/블록)
- ✅ 문자열/char 리터럴 처리
- ✅ 숫자 파싱 (정수/실수)
- ✅ 5개 테스트 통과

**코드**: `src/tokenizer.rs` (670줄)

---

### Phase 5.2: 파서 상세 구현 ⏳
**목표**: Recursive Descent Parser 완성

**구현할 함수**:
```rust
impl Parser {
    pub fn parse(&mut self) -> Result<Program, Vec<ParseError>>

    // Top-level items
    fn parse_item(&mut self) -> Result<Item, ParseError>
    fn parse_fn_decl(&mut self) -> Result<FnDecl, ParseError>
    fn parse_struct_decl(&mut self) -> Result<StructDecl, ParseError>
    fn parse_impl_block(&mut self) -> Result<ImplBlock, ParseError>

    // Statements
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_var_decl(&mut self) -> Result<Stmt, ParseError>
    fn parse_const_decl(&mut self) -> Result<Stmt, ParseError>
    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_for_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_loop_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_match_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError>
    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError>

    // Expressions (Precedence climbing)
    fn parse_expr(&mut self) -> Result<Expr, ParseError>
    fn parse_assignment(&mut self) -> Result<Expr, ParseError>
    fn parse_logical_or(&mut self) -> Result<Expr, ParseError>
    fn parse_logical_and(&mut self) -> Result<Expr, ParseError>
    fn parse_equality(&mut self) -> Result<Expr, ParseError>
    fn parse_comparison(&mut self) -> Result<Expr, ParseError>
    fn parse_additive(&mut self) -> Result<Expr, ParseError>
    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError>
    fn parse_unary(&mut self) -> Result<Expr, ParseError>
    fn parse_postfix(&mut self) -> Result<Expr, ParseError>
    fn parse_primary(&mut self) -> Result<Expr, ParseError>

    // Type parsing
    fn parse_type(&mut self) -> Result<RustType, ParseError>
}
```

**예상 코드**: ~800줄
**테스트**: 15+ 테스트 (함수, if, 루프, 연산자 우선순위, etc)

---

### Phase 5.3: 타입 검사기 상세 구현 ⏳
**목표**: 타입 추론 및 검증

**구현할 함수**:
```rust
impl TypeChecker {
    pub fn check(&mut self, program: &Program) -> Result<(), Vec<TypeCheckError>>

    // Type checking
    fn check_item(&mut self, item: &Item) -> Result<(), TypeCheckError>
    fn check_fn_decl(&mut self, fn_decl: &FnDecl) -> Result<(), TypeCheckError>
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeCheckError>
    fn check_expr(&mut self, expr: &Expr) -> Result<RustType, TypeCheckError>

    // Type inference
    fn infer_literal_type(&self, lit: &Literal) -> RustType
    fn infer_binary_op_type(&self, op: &str, lhs: &RustType, rhs: &RustType) -> RustType
    fn infer_unary_op_type(&self, op: &str, operand: &RustType) -> RustType

    // Type compatibility
    pub fn types_compatible(&self, lhs: &RustType, rhs: &RustType) -> bool
    fn types_equal(&self, lhs: &RustType, rhs: &RustType) -> bool

    // Scope management
    fn push_scope(&mut self)
    fn pop_scope(&mut self)
    fn declare_symbol(&mut self, name: String, ty: RustType)
    fn lookup_symbol(&self, name: &str) -> Option<RustType>
    fn collect_functions(&mut self, program: &Program)
}
```

**예상 코드**: ~900줄
**테스트**: 15+ 테스트 (타입 추론, 호환성 검사, 스코핑, etc)

---

### Phase 5.4: 코드 생성기 상세 구현 ⏳
**목표**: AST를 C 코드로 변환

**구현할 함수**:
```rust
impl CodeGenerator {
    pub fn generate(&mut self, program: &Program) -> String

    // Code generation
    fn gen_program(&mut self, program: &Program)
    fn gen_item(&mut self, item: &Item)
    fn gen_fn_decl(&mut self, fn_decl: &FnDecl)
    fn gen_fn_prototype(&mut self, fn_decl: &FnDecl)
    fn gen_struct_decl(&mut self, struct_decl: &StructDecl)
    fn gen_stmt(&mut self, stmt: &Stmt)
    fn gen_block(&mut self, stmts: &[Stmt])
    fn gen_expr(&mut self, expr: &Expr) -> String

    // Specific statement generators
    fn gen_var_decl(&mut self, name: &str, ty: &Option<RustType>, init: &Option<Expr>)
    fn gen_if_stmt(&mut self, cond: &Expr, then_body: &[Stmt], else_body: &Option<Vec<Stmt>>)
    fn gen_for_stmt(&mut self, var: &str, iter: &Expr, body: &[Stmt])
    fn gen_while_stmt(&mut self, cond: &Expr, body: &[Stmt])
    fn gen_loop_stmt(&mut self, body: &[Stmt])
    fn gen_match_stmt(&mut self, expr: &Expr, arms: &[(String, Vec<Stmt>)])
    fn gen_return_stmt(&mut self, expr: &Option<Expr>)

    // Expression generation
    fn gen_literal(&self, lit: &Literal) -> String
    fn gen_binary_op(&mut self, lhs: &Expr, op: &str, rhs: &Expr) -> String
    fn gen_unary_op(&mut self, op: &str, operand: &Expr) -> String
    fn gen_call(&mut self, func: &str, args: &[Expr]) -> String
    fn gen_method_call(&mut self, object: &Expr, method: &str, args: &[Expr]) -> String
    fn gen_field_access(&mut self, object: &Expr, field: &str) -> String
    fn gen_index(&mut self, object: &Expr, index: &Expr) -> String

    // Type conversion
    pub fn rust_type_to_c(&self, ty: &RustType) -> String
    fn operator_to_c(&self, op: &str) -> String

    // Utility
    fn gen_temp_var(&mut self) -> String
    fn gen_label(&mut self) -> String
    fn indent(&mut self)
    fn unindent(&mut self)
    fn indent_str(&self) -> String
}
```

**예상 코드**: ~700줄
**테스트**: 15+ 테스트 (함수, if, 루프, 표현식, C 타입 변환, etc)

---

### Phase 5.5: 통합 및 테스트 ⏳
**목표**: 전체 파이프라인 통합 테스트

**테스트 항목**:
- ✅ 개별 모듈 테스트 (이미 포함됨)
- ⏳ 파이프라인 통합 테스트
- ⏳ 실제 Rust 프로그램 컴파일 테스트
- ⏳ 생성된 C 코드 컴파일 테스트
- ⏳ 자체호스팅 테스트 (컴파일러가 자신을 컴파일)

**예상 코드**: `tests/integration_tests.rs` (~500줄)

---

## 📊 구현 메트릭

### 현재 코드량
| 모듈 | 줄 | 상태 |
|------|-----|------|
| Cargo.toml | 16 | ✅ |
| lib.rs | 43 | ✅ |
| main.rs | 105 | ✅ |
| tokenizer.rs | 670 | ✅ |
| ast.rs | 240 | ✅ |
| parser.rs | 90 | 🔄 |
| type_checker.rs | 95 | 🔄 |
| codegen.rs | 135 | 🔄 |
| compiler.rs | 110 | ✅ |
| **Total** | **1,504** | |

### 최종 예상 코드량 (완성시)
| 모듈 | 현재 | 최종 | 추가 |
|------|------|------|------|
| parser.rs | 90 | 890 | +800 |
| type_checker.rs | 95 | 995 | +900 |
| codegen.rs | 135 | 835 | +700 |
| integration_tests.rs | 0 | 500 | +500 |
| **Total** | **1,504** | **5,819** | **+3,315** |

---

## 🔄 구현 순서

```
1. Phase 5.1: Tokenizer ✅
   ↓
2. Phase 5.2: Parser ⏳
   ↓
3. Phase 5.3: TypeChecker ⏳
   ↓
4. Phase 5.4: CodeGen ⏳
   ↓
5. Phase 5.5: Integration Tests ⏳
   ↓
6. Self-hosting Test
   (Compile simple_tokenizer.rs with minrust)
   ↓
7. Phase 5 Complete ✅
```

---

## ✅ 검증 체크리스트

### 개발 중
- [ ] Parser 구현 (예정 ~800줄)
- [ ] 15+ Parser 테스트
- [ ] TypeChecker 구현 (예정 ~900줄)
- [ ] 15+ TypeChecker 테스트

### 최종 검증
- [ ] CodeGen 구현 (예정 ~700줄)
- [ ] 15+ CodeGen 테스트
- [ ] Integration 테스트 (500줄)
- [ ] 50+ 통합 테스트 통과
- [ ] 컴파일러가 자신을 컴파일
- [ ] 생성된 C 코드가 gcc로 컴파일됨
- [ ] 모든 테스트 통과

---

## 🎯 다음 단계 후 계획

Phase 5 완료 후:
- Phase 6: 고급 최적화 (1,590줄 계획)
- Phase 7: 고급 기능 (1,000줄 계획)
- Phase 8: 생태계 구축 (1,000줄 계획)

**최종 목표**: 12,400+ 줄 (현재 진행도: 14,690줄 ✅ 초과 달성)

---

## 💾 빌드 및 실행

### Rust 환경 설정
```bash
# Rust 설치 (필요시)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 기본 도구모음 설정
rustup default stable
```

### 빌드
```bash
cd src_rust
cargo build --release
```

### 실행
```bash
# CLI 직접 실행
cargo run -- compile test.fl

# 또는 바이너리로
./target/release/minrustc compile test.fl
```

### 테스트
```bash
# 모든 테스트 실행
cargo test

# 특정 모듈 테스트
cargo test tokenizer
cargo test parser

# 통합 테스트만
cargo test --test integration_tests
```

---

## 📝 구현 팁

### Rust 패턴
- `enum` for token types and AST nodes
- `match` for pattern matching
- `Box<T>` for recursive types
- `Vec<T>` for collections
- `HashMap<K,V>` for symbol tables
- `Option<T>` and `Result<T,E>` for error handling

### 메모리 관리
- Rust의 소유권 시스템 활용
- 순환 참조 주의
- 필요시 `Rc<RefCell<T>>` 사용 (드물게)

### 성능 최적화
- `&str` instead of `String` where possible
- Pre-allocate vectors with capacity
- Use `.clone()` sparingly
- Profile with `cargo flamegraph`

---

## 참고 자료

- **이전 구현**: `src/tokenizer.jl` (Julia), `src/parser.jl` 등
- **표준 라이브러리**: `src/minrust_stdlib.h`
- **테스트 전략**: `test/test_runner.jl`

---

## 상태 업데이트

**현재 완성도**: Phase 5 초반 (토크나이저 완성, 파서 스켈레톤 완성)
**예상 완성 시간**: 2-3 세션 (각 세션당 약 1 모듈 완성)
**마일스톤**: 컴파일러가 자신을 컴파일하는 순간 ✨
