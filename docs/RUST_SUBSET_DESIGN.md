# 🦀 MinRust: Rust 언어 서브셋 설계

**프로젝트**: MinRust (Simplified Rust Compiler)
**버전**: 1.0
**작성일**: 2026-03-11 UTC+9

---

## 📖 개요

MinRust는 Rust 언어의 핵심 기능만 추출하여 구현한 교육용 컴파일러입니다. 실제 Rust의 모든 기능을 지원하지는 않지만, 프로그래밍의 기본 개념과 타입 시스템, 소유권 개념(기초)을 학습할 수 있습니다.

---

## ✅ 지원하는 기능

### 1. 기본 타입 (7가지)

```rust
// 정수 타입
let x: i32 = 42;        // 32비트 정수
let y: i64 = 1000;      // 64비트 정수
let u: u32 = 100;       // 부호없는 정수

// 부동소수
let pi: f64 = 3.14;     // 64비트 부동소수

// 논리 & 문자
let b: bool = true;     // 불린
let c: char = 'A';      // 문자

// 컬렉션
let s: String = "hello".to_string();  // 동적 문자열
let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 고정 배열
let v: Vec<i32> = vec![1, 2, 3];     // 동적 벡터
```

### 2. 변수 선언

```rust
// 불변 바인딩
let x = 42;
let x: i32 = 42;                  // 타입 명시

// 가변 바인딩
let mut y = 100;
y = 200;                          // 변경 가능

// 상수
const MAX_SIZE: usize = 1000;     // 컴파일타임 상수
```

### 3. 함수

```rust
// 기본 함수
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 리턴 타입 없음 (void)
fn print_value(x: i32) {
    println!("{}", x);
}

// 매개변수 없음
fn get_constant() -> i32 {
    42
}

// 제너릭 함수 (기초 - Phase 7)
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### 4. 제어 흐름

```rust
// if-else 문
if x > 0 {
    println!("positive");
} else if x < 0 {
    println!("negative");
} else {
    println!("zero");
}

// 루프: for
for i in 0..10 {
    println!("{}", i);
}

// 루프: while
while x > 0 {
    x = x - 1;
}

// 루프: loop (무한)
loop {
    if condition { break; }
}

// 루프 제어
for i in 0..10 {
    if i == 5 { continue; }
    if i == 8 { break; }
    println!("{}", i);
}

// match 표현식
match value {
    0 => println!("zero"),
    1 | 2 | 3 => println!("1-3"),
    n if n > 10 => println!("large"),
    _ => println!("other"),
}
```

### 5. 구조체 (기본)

```rust
// 구조체 정의
struct Point {
    x: i32,
    y: i32,
}

// 구조체 인스턴스
let p = Point { x: 10, y: 20 };

// 필드 접근
let px = p.x;

// 메서드 (Phase 5+)
impl Point {
    fn distance(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

let dist = p.distance();
```

### 6. 배열 및 벡터 연산

```rust
// 배열
let arr = [1, 2, 3, 4, 5];
let len = arr.len();              // 5
let first = arr[0];               // 1

// 벡터
let mut v = vec![1, 2, 3];
v.push(4);                        // [1, 2, 3, 4]
let val = v.pop();                // Some(4)
let elem = v[0];                  // 1

// 슬라이싱
let slice = &arr[1..3];           // [2, 3]
```

### 7. 소유권 (기초)

```rust
// 이동 (Move)
let s1 = String::from("hello");
let s2 = s1;                      // s1 소유권이 s2로 이동
// println!("{}", s1);            // 오류: s1은 더 이상 유효하지 않음

// 참조 (Borrow)
let s1 = String::from("hello");
let s2 = &s1;                     // s1을 빌려줌
println!("{}", s1);               // OK, s1은 여전히 유효
println!("{}", s2);               // OK

// 가변 참조
let mut s1 = String::from("hello");
let s2 = &mut s1;                 // 가변 참조
s2.push_str(" world");
```

### 8. 표준 함수

```rust
// I/O
println!("{}", value);            // 표준출력
print!("text");                   // 개행 없음
eprint!("{}", error);             // 표준에러

// 문자열
s.len()                           // 길이
s.to_uppercase()                  // 대문자
s.to_lowercase()                  // 소문자
s.trim()                          // 공백 제거
s.starts_with("prefix")           // 시작 문자 확인
s.ends_with("suffix")             // 끝 문자 확인
s.find("substr")                  // 부분문자 찾기
s.replace("old", "new")           // 문자열 교체

// 정수
n.abs()                           // 절댓값
max(a, b)                         // 최댓값
min(a, b)                         // 최솟값
(n as f64).sqrt()                 // 제곱근

// 벡터
v.iter()                          // 반복자
v.map(|x| x * 2)                 // 맵
v.filter(|x| x > 0)              // 필터
```

### 9. 에러 처리 (기초)

```rust
// Option
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

match divide(10, 2) {
    Some(result) => println!("{}", result),
    None => println!("division by zero"),
}

// Result (간단한 형태)
fn parse_int(s: &str) -> Result<i32, String> {
    // 파싱 시도
}
```

### 10. 타입 캐스팅

```rust
let x = 42i32;
let y = x as i64;                 // i32 → i64
let z = (x as f64) * 3.14;        // i32 → f64 변환
let b = x != 0;                   // i32 → bool
```

---

## ❌ 지원하지 않는 기능

### 1. 고급 소유권
- ❌ 라이프타임 표기 (`'a`)
- ❌ 복잡한 차용 규칙
- ❌ 드롭 구현

### 2. 고급 타입
- ❌ 트레이트 객체 (dyn Trait)
- ❌ 고급 제너릭 (where 절)
- ❌ 함수형 타입 (fn pointer, closure 고급)

### 3. 패턴 매칭 (고급)
- ❌ 중첩된 구조화 (destructuring)
- ❌ 범위 패턴 (5..=10)
- ❌ Or 패턴 (복잡한 형태)

### 4. 제너릭 & 트레이트 (고급)
- ❌ 트레이트 바운드 (제한된 형태만 지원)
- ❌ 고급 제너릭 (associated types)
- ❌ 폴리모피즘 (vtable)

### 5. 매크로
- ❌ `macro_rules!` (선택)
- ❌ 프로시저 매크로
- 단, `println!`, `vec!` 같은 기본 매크로는 지원

### 6. 모듈 시스템 (고급)
- ❌ `mod` 네임스페이스
- ❌ `use` 경로 (기초만 지원)
- ❌ crate 시스템

### 7. 비동기 프로그래밍
- ❌ `async`/`await`
- ❌ Future
- ❌ Pin

### 8. 매크로 관련
- ❌ 절차 매크로
- ❌ 속성 매크로
- ❌ 선언 매크로 (고급)

---

## 📊 예제: "Hello, Rust!" 프로그램

```rust
// main.rs - MinRust로 컴파일 가능

fn main() {
    // 변수 선언
    let name = "Rust";
    let count: i32 = 5;

    // 루프
    for i in 0..count {
        println!("Hello, {}! ({})", name, i);
    }

    // 함수 호출
    let result = add(10, 20);
    println!("10 + 20 = {}", result);

    // 구조체
    let point = Point { x: 3, y: 4 };
    println!("Distance: {}", point.distance());

    // match
    match result {
        30 => println!("Exact!"),
        _ => println!("Other"),
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}
```

---

## 🎯 Phase별 기능 확장

### Phase 1-2 (기초)
✅ 변수, 함수, 제어 흐름, 기본 구조체

### Phase 3 (자체호스팅)
✅ Tokenizer 부트스트랩

### Phase 4 (표준 라이브러리)
✅ I/O, 문자열, 배열 연산, 정수 함수

### Phase 5 (Rust 자체호스팅)
✅ 완전한 컴파일러 (Rust 구현)

### Phase 6 (최적화)
✅ 상수 전파, 데드 코드 제거, 루프 최적화

### Phase 7 (고급 기능)
✅ 제너릭, 트레이트, 패턴 매칭 강화, 매크로 (기초)

### Phase 8 (생태계)
✅ VSCode 통합, Playground, Cargo.toml, 문서

---

## 📈 언어 특성 요약

| 특성 | MinRust | 지원 형태 |
|------|---------|----------|
| 정적 타입 | ✅ | 명시적, 추론 가능 |
| 강한 타입 | ✅ | 타입 호환성 엄격 |
| 소유권 | ⚠️ | 기초만 (라이프타임 없음) |
| 함수형 | ⚠️ | map/filter (클로저 제한) |
| 객체지향 | ⚠️ | struct + impl (트레이트 기초) |
| 제너릭 | ⚠️ | 기초 (Phase 7+) |
| 에러 처리 | ✅ | Option/Result (간단) |
| 메모리 | ✅ | 자동 (GC 유사) |
| 성능 | ✅ | LLVM IR 컴파일 |

---

## 💡 학습 목표

이 프로젝트를 통해 학습할 수 있는 내용:

1. **컴파일러 구조**: 6단계 파이프라인 이해
2. **타입 시스템**: 정적 타입 검증
3. **코드생성**: AST → IR → 기계 코드
4. **제어 흐름**: if, loop, match 분석
5. **메모리 관리**: 소유권 개념 (기초)
6. **최적화**: 상수 전파, DCE, 루프 최적화
7. **에코시스템**: IDE 통합, 패키지 관리

---

**다음 문서**: `COMPILER_ARCHITECTURE.md` (6단계 파이프라인)
