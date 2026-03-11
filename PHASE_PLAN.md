# 🦀 Rust 컴파일러 프로젝트 (MinRust)

**프로젝트명**: MinRust - Simplified Rust Compiler
**시작일**: 2026-03-11 UTC+9
**목표**: FreeLiulia와 동일한 방식으로 Rust 언어 서브셋 컴파일러 구축

---

## 📋 8-Phase 계획

### Phase 1: 설계 & 분석 (목표: 3,000줄)
**목표**: Rust 언어 서브셋 설계, 컴파일러 아키텍처 분석

**구현**:
- `docs/RUST_SUBSET_DESIGN.md` - 지원할 Rust 기능 명시
- `docs/COMPILER_ARCHITECTURE.md` - 6단계 파이프라인
- `docs/IMPLEMENTATION_ROADMAP.md` - 각 단계별 구현 계획

**핵심 기능**:
```rust
// 지원할 기능
fn main() {
    let x: i32 = 42;
    let y = x + 10;

    if y > 50 {
        println!("{}", y);
    }

    for i in 0..5 {
        println!("{}", i);
    }
}
```

**비지원** (단순화):
- 트레이트, 제너릭 (Phase 2에서만 기초)
- 소유권/차용 (참조만 지원)
- 매크로, 모듈 시스템
- 라이프타임 (암시적 처리)

---

### Phase 2: 컴파일러 구현 (목표: 3,500줄)

#### 2-1: Tokenizer (500줄)
**파일**: `src/tokenizer.jl`
- 60+ 토큰 타입
- 키워드 인식: fn, let, const, if, else, for, while, loop, match, struct, impl, return, pub, mut, etc.
- 연산자: +, -, *, /, %, ==, !=, <, >, <=, >=, &&, ||, !, &, |, ^, <<, >>, =, etc.
- 문자열/문자 리터럴 (이스케이프 처리)
- 숫자 (정수, 부동소수, 16진수, 2진수)

#### 2-2: Parser (1,000줄)
**파일**: `src/parser.jl`
- Recursive Descent 파서
- 50+ AST 노드 타입
- Expression parsing: Binary op, unary op, call, member access, index access
- Statement parsing: let, const, if, for, while, loop, match, fn decl, struct decl
- Type parsing: i32, i64, f64, bool, &T, [T], String, Vec<T>

#### 2-3: AST (400줄)
**파일**: `src/ast.jl`
- 노드 정의: Program, FnDecl, VarDecl, StructDecl, Block, If, For, While, Match, etc.
- Type enum: Integer, Float, Bool, String, Reference, Array, Vector, Struct
- Expression nodes: Literal, Identifier, BinaryOp, UnaryOp, Call, MemberAccess, Index

#### 2-4: Type Checking (1,000줄)
**파일**: `src/type_checker.jl`
- Type inference
- Type compatibility checking
- Reference validation
- Function signature validation
- Struct field validation
- Error reporting with line numbers

#### 2-5: Code Generation (800줄)
**파일**: `src/codegen.jl`
- AST → LLVM IR (간단한 형태)
- 변수 스택 프레임
- 함수 호출 규약
- 제어 흐름 (if, for, while)
- 메모리 관리 (기초)

#### 2-6: 통합 컴파일러 (300줄)
**파일**: `src/compiler.jl`
```
Pipeline:
  [Input Rust] → [Tokenize] → [Parse] → [Type Check] → [CodeGen] → [Output IR]
```

**테스트** (800줄):
- `test/tokenizer_test.jl` - 40개 테스트
- `test/parser_test.jl` - 30개 테스트
- `test/type_checker_test.jl` - 35개 테스트
- `test/codegen_test.jl` - 25개 테스트

**보고서**: `PHASE_2_COMPILER_REPORT.md`

---

### Phase 3: 자체호스팅 검증 (목표: 600줄)

**목표**: Tokenizer의 기본 부분을 Rust로 다시 작성 후 원래 Julia 컴파일러로 검증

**구현**:
- `src/simple_tokenizer.rs` - 기본 토크나이저 (100줄)
- `test/self_hosting_test.jl` - Julia 컴파일러로 Rust 코드 검증 (200줄)
- 부트스트랩 검증 (300줄)

**보고서**: `PHASE_3_SELF_HOSTING_REPORT.md`

---

### Phase 4: 표준 라이브러리 (목표: 1,500줄)

**구현**: `src/minrust_stdlib.h` (C)
- 40+ 함수

**카테고리**:
```
기본 I/O (8개):
  println!(), print!(), eprint!(), eprintln!()
  read_line(), input(), output()

문자열 (12개):
  len(), chars(), bytes(), to_string()
  to_uppercase(), to_lowercase(), trim()
  split(), replace(), find(), starts_with(), ends_with()

배열/벡터 (10개):
  len(), push(), pop(), get(), iter()
  map(), filter(), fold(), zip(), collect()

정수 연산 (8개):
  abs(), max(), min(), pow(), sqrt()
  gcd(), lcm(), factorial()

파일 I/O (6개):
  open(), read(), write(), close()
  exists(), delete()
```

**테스트**: 60개 테스트 (C)
**보고서**: `STDLIB.md`, `PHASE_4_STDLIB_REPORT.md`

---

### Phase 5: 자체호스팅 컴파일러 (목표: 1,200줄)

**목표**: MinRust 컴파일러를 Rust로 완전히 재구현

**구현**:
- `src/lib.rs` - 핵심 라이브러리
- `src/tokenizer.rs` (300줄)
- `src/parser.rs` (400줄)
- `src/type_checker.rs` (300줄)
- `src/codegen.rs` (200줄)

**테스트**: 40개
**보고서**: `PHASE_5_SELF_HOSTING_REPORT.md`

---

### Phase 6: 고급 최적화 (목표: 1,000줄)

**최적화 기법**:
1. 상수 전파 (Constant Propagation)
2. 데드 코드 제거 (Dead Code Elimination)
3. 루프 최적화 (Loop Unrolling, Hoisting)
4. 함수 인라인화
5. 타입 기반 최적화

**구현**: `src/optimizer.rs` (400줄)
**테스트**: 30개
**보고서**: `PHASE_6_OPTIMIZATION_REPORT.md`

---

### Phase 7: 고급 기능 (목표: 800줄)

**추가 기능**:
1. 제너릭 (기초)
2. 트레이트 (기초)
3. 패턴 매칭 강화
4. 모듈 시스템 (기초)
5. 매크로 (기초)

**테스트**: 25개
**보고서**: `PHASE_7_ADVANCED_FEATURES_REPORT.md`

---

### Phase 8: 생태계 구축 (목표: 800줄)

**구현**:
1. **VSCode 문법 강조** (80줄)
   - `docs/minrust.tmLanguage.json`
   - Rust 키워드, 타입, 매크로 하이라이팅

2. **온라인 Playground** (300줄)
   - `docs/playground.html`
   - 코드 에디터 + 컴파일 시뮬레이터
   - 예제 로드

3. **Cargo 패키지 시스템** (150줄)
   - `Cargo.toml` 파서
   - 의존성 관리
   - 예제: `examples/hello_cargo/`

4. **설정 가이드** (100줄)
   - `docs/SETUP.md`
   - IDE 통합
   - 문제 해결

**보고서**: `PHASE_8_ECOSYSTEM_REPORT.md`

---

## 📊 예상 결과

```
Phase 1: 분석 & 설계 ............... 3,000줄
Phase 2: 컴파일러 구현 ............ 3,500줄
Phase 3: 자체호스팅 검증 ........... 600줄
Phase 4: 표준 라이브러리 .......... 1,500줄
Phase 5: 자체호스팅 컴파일러 ...... 1,200줄
Phase 6: 고급 최적화 ............. 1,000줄
Phase 7: 고급 기능 ................. 800줄
Phase 8: 생태계 구축 ............... 800줄
────────────────────────────────
합계 ............................. 12,400줄

테스트: 200+ (모두 통과)
문서: 2,000+ 줄
커밋: 16+개
```

---

## 🚀 시작 옵션

**어디서부터 시작할까요?**

```
1️⃣  Phase 1 (설계 & 분석)
    ➜ Rust 서브셋 정의, 아키텍처 설계

2️⃣  Phase 1-2 (설계 + 컴파일러)
    ➜ Tokenizer, Parser, Type Checker 구현

3️⃣  Phase 1-3 (설계 + 컴파일러 + 자체호스팅)
    ➜ 완전한 컴파일 파이프라인

4️⃣  Phase 1-5 (설계 + 컴파일러 + 자체호스팅 + 표준라이브러리)
    ➜ 기본 완성 (총 9,800줄)

5️⃣  Phase 1-8 (전체, 최종)
    ➜ 완전한 생태계 (총 12,400줄)

⭐ All (모두, 권장)
    ➜ 전 단계를 순서대로 구현
```

---

**선택**: `1` / `2` / `3` / `4` / `5` / `⭐ All` ?
