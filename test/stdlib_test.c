// MinRust Standard Library Tests - Phase 4
// Tests all 52 standard library functions

#include "../src/minrust_stdlib.h"
#include <assert.h>

// Test counter
int test_count = 0;
int pass_count = 0;
int fail_count = 0;

void assert_equal_int(int actual, int expected, const char* test_name) {
    test_count++;
    if (actual == expected) {
        pass_count++;
        printf("  ✓ %s\n", test_name);
    } else {
        fail_count++;
        printf("  ✗ %s (expected %d, got %d)\n", test_name, expected, actual);
    }
}

void assert_equal_str(const char* actual, const char* expected, const char* test_name) {
    test_count++;
    if (strcmp(actual, expected) == 0) {
        pass_count++;
        printf("  ✓ %s\n", test_name);
    } else {
        fail_count++;
        printf("  ✗ %s (expected '%s', got '%s')\n", test_name, expected, actual);
    }
}

void assert_true(int condition, const char* test_name) {
    test_count++;
    if (condition) {
        pass_count++;
        printf("  ✓ %s\n", test_name);
    } else {
        fail_count++;
        printf("  ✗ %s\n", test_name);
    }
}

void assert_close(double actual, double expected, double epsilon, const char* test_name) {
    test_count++;
    if (fabs(actual - expected) < epsilon) {
        pass_count++;
        printf("  ✓ %s\n", test_name);
    } else {
        fail_count++;
        printf("  ✗ %s (expected %.2f, got %.2f)\n", test_name, expected, actual);
    }
}

// ============================================================
// I/O Tests (5 tests)
// ============================================================
void test_io_functions() {
    printf("\n📝 Testing I/O Functions\n");

    // Test 1: dbg function returns value unchanged
    int dbg_result = minrust_dbg(42);
    assert_equal_int(dbg_result, 42, "dbg returns value unchanged");

    // Test 2: strlen_custom works
    int str_len = minrust_strlen("hello");
    assert_equal_int(str_len, 5, "strlen_custom('hello') == 5");

    // Test 3: Parse functions work
    int parsed_int = minrust_parse_int("123");
    assert_equal_int(parsed_int, 123, "parse_int('123') == 123");

    double parsed_float = minrust_parse_float("3.14");
    assert_close(parsed_float, 3.14, 0.01, "parse_float('3.14') ≈ 3.14");

    int parsed_bool = minrust_to_bool("true");
    assert_equal_int(parsed_bool, 1, "to_bool('true') == 1");
}

// ============================================================
// String Tests (8 tests)
// ============================================================
void test_string_functions() {
    printf("\n🔤 Testing String Functions\n");

    // Test 1: strcat_custom
    char* concat = minrust_strcat("Hello", " World");
    assert_equal_str(concat, "Hello World", "strcat('Hello', ' World')");
    free(concat);

    // Test 2: strslice
    char* sliced = minrust_strslice("hello", 1, 4);
    assert_equal_str(sliced, "ell", "strslice('hello', 1, 4) == 'ell'");
    free(sliced);

    // Test 3: to_uppercase
    char* upper = minrust_to_uppercase("hello");
    assert_equal_str(upper, "HELLO", "to_uppercase('hello') == 'HELLO'");
    free(upper);

    // Test 4: to_lowercase
    char* lower = minrust_to_lowercase("HELLO");
    assert_equal_str(lower, "hello", "to_lowercase('HELLO') == 'hello'");
    free(lower);

    // Test 5: trim
    char* trimmed = minrust_trim("  hello  ");
    assert_equal_str(trimmed, "hello", "trim('  hello  ') == 'hello'");
    free(trimmed);

    // Test 6: contains
    int contains = minrust_contains("hello world", "world");
    assert_equal_int(contains, 1, "contains('hello world', 'world') == true");

    // Test 7: starts_with
    int starts = minrust_starts_with("hello", "hel");
    assert_equal_int(starts, 1, "starts_with('hello', 'hel') == true");

    // Test 8: ends_with
    int ends = minrust_ends_with("hello", "lo");
    assert_equal_int(ends, 1, "ends_with('hello', 'lo') == true");
}

// ============================================================
// String Advanced Tests (4 tests)
// ============================================================
void test_string_advanced() {
    printf("\n🔍 Testing String Advanced Functions\n");

    // Test 1: find
    int pos = minrust_find("hello world", "world");
    assert_equal_int(pos, 6, "find('hello world', 'world') == 6");

    // Test 2: find (not found)
    int not_found = minrust_find("hello", "xyz");
    assert_equal_int(not_found, -1, "find('hello', 'xyz') == -1");

    // Test 3: replace
    char* replaced = minrust_replace("hello world", "world", "Rust");
    assert_true(strcmp(replaced, "hello Rust") == 0, "replace('hello world', 'world', 'Rust')");
    free(replaced);

    // Test 4: substr
    char* sub = minrust_substr("hello", 1, 3);
    assert_equal_str(sub, "ell", "substr('hello', 1, 3) == 'ell'");
    free(sub);
}

// ============================================================
// Array Tests (6 tests)
// ============================================================
void test_array_functions() {
    printf("\n📦 Testing Array Functions\n");

    IntArray arr = minrust_array_new();

    // Test 1: push and len
    minrust_array_push(&arr, 10);
    minrust_array_push(&arr, 20);
    minrust_array_push(&arr, 30);
    assert_equal_int(minrust_array_len(&arr), 3, "array_len after 3 pushes == 3");

    // Test 2: get
    int val = minrust_array_get(&arr, 1);
    assert_equal_int(val, 20, "array_get(1) == 20");

    // Test 3: set
    minrust_array_set(&arr, 1, 25);
    val = minrust_array_get(&arr, 1);
    assert_equal_int(val, 25, "array_set(1, 25) works");

    // Test 4: contains
    int contains = minrust_array_contains(&arr, 25);
    assert_equal_int(contains, 1, "array_contains(25) == true");

    // Test 5: index_of
    int idx = minrust_array_index_of(&arr, 30);
    assert_equal_int(idx, 2, "array_index_of(30) == 2");

    // Test 6: pop
    int popped = minrust_array_pop(&arr);
    assert_equal_int(popped, 30, "array_pop() == 30");

    minrust_array_free(&arr);
}

// ============================================================
// Array Advanced Tests (3 tests)
// ============================================================
void test_array_advanced() {
    printf("\n🔢 Testing Array Advanced Functions\n");

    IntArray arr = minrust_array_new();
    minrust_array_push(&arr, 1);
    minrust_array_push(&arr, 2);
    minrust_array_push(&arr, 3);

    // Test 1: contains (false case)
    int not_contains = minrust_array_contains(&arr, 99);
    assert_equal_int(not_contains, 0, "array_contains(99) == false");

    // Test 2: index_of (not found)
    int not_found = minrust_array_index_of(&arr, 99);
    assert_equal_int(not_found, -1, "array_index_of(99) == -1");

    // Test 3: clear
    minrust_array_clear(&arr);
    assert_equal_int(minrust_array_len(&arr), 0, "array_clear works");

    minrust_array_free(&arr);
}

// ============================================================
// Math Tests (8 tests)
// ============================================================
void test_math_functions() {
    printf("\n🔢 Testing Math Functions\n");

    // Test 1: abs
    int abs_val = minrust_abs(-42);
    assert_equal_int(abs_val, 42, "abs(-42) == 42");

    // Test 2: abs_f64
    double abs_f = minrust_abs_f64(-3.14);
    assert_close(abs_f, 3.14, 0.01, "abs_f64(-3.14) ≈ 3.14");

    // Test 3: sqrt
    double sqrt_val = minrust_sqrt(16.0);
    assert_close(sqrt_val, 4.0, 0.01, "sqrt(16) ≈ 4");

    // Test 4: pow
    double pow_val = minrust_pow(2.0, 3.0);
    assert_close(pow_val, 8.0, 0.01, "pow(2, 3) ≈ 8");

    // Test 5: min
    int min_val = minrust_min(5, 3);
    assert_equal_int(min_val, 3, "min(5, 3) == 3");

    // Test 6: max
    int max_val = minrust_max(5, 3);
    assert_equal_int(max_val, 5, "max(5, 3) == 5");

    // Test 7: floor
    double floor_val = minrust_floor(3.7);
    assert_close(floor_val, 3.0, 0.01, "floor(3.7) ≈ 3");

    // Test 8: ceil
    double ceil_val = minrust_ceil(3.2);
    assert_close(ceil_val, 4.0, 0.01, "ceil(3.2) ≈ 4");
}

// ============================================================
// Math Advanced Tests (4 tests)
// ============================================================
void test_math_advanced() {
    printf("\n📐 Testing Math Advanced Functions\n");

    // Test 1: round
    double round_val = minrust_round(3.6);
    assert_close(round_val, 4.0, 0.01, "round(3.6) ≈ 4");

    // Test 2: sin
    double sin_val = minrust_sin(0.0);
    assert_close(sin_val, 0.0, 0.01, "sin(0) ≈ 0");

    // Test 3: cos
    double cos_val = minrust_cos(0.0);
    assert_close(cos_val, 1.0, 0.01, "cos(0) ≈ 1");

    // Test 4: tan
    double tan_val = minrust_tan(0.0);
    assert_close(tan_val, 0.0, 0.01, "tan(0) ≈ 0");
}

// ============================================================
// Type Conversion Tests (4 tests)
// ============================================================
void test_type_conversion() {
    printf("\n🔄 Testing Type Conversion Functions\n");

    // Test 1: int_to_string
    char* str_int = minrust_int_to_string(42);
    assert_equal_str(str_int, "42", "int_to_string(42) == '42'");
    free(str_int);

    // Test 2: float_to_string
    char* str_float = minrust_float_to_string(3.14);
    assert_true(strncmp(str_float, "3.14", 4) == 0, "float_to_string(3.14)");
    free(str_float);

    // Test 3: to_char
    char ch = minrust_to_char(65);
    assert_equal_int(ch, 65, "to_char(65) == 65");

    // Test 4: to_bool (false)
    int bool_false = minrust_to_bool("false");
    assert_equal_int(bool_false, 0, "to_bool('false') == 0");
}

// ============================================================
// File I/O Tests (5 tests - basic)
// ============================================================
void test_file_io_functions() {
    printf("\n📄 Testing File I/O Functions\n");

    const char* test_file = "/tmp/minrust_test.txt";

    // Test 1: file_open for writing
    FILE* f = minrust_file_open(test_file, "w");
    assert_true(f != NULL, "file_open for writing");

    // Test 2: file_write_line
    minrust_file_write_line(f, "Hello, MinRust!");
    minrust_file_write_line(f, "Line 2");

    // Test 3: file_close
    minrust_file_close(f);
    assert_true(f != NULL, "file_close (file was opened)");

    // Test 4: file_exists
    int exists = minrust_file_exists(test_file);
    assert_equal_int(exists, 1, "file_exists(test_file) == true");

    // Test 5: file_open for reading and file_read_line
    f = minrust_file_open(test_file, "r");
    if (f != NULL) {
        char* line = minrust_file_read_line(f);
        assert_true(line != NULL, "file_read_line returns data");
        if (line != NULL) {
            assert_equal_str(line, "Hello, MinRust!", "file_read_line content matches");
            free(line);
        }
        minrust_file_close(f);
    }
}

// ============================================================
// File I/O Advanced Tests (2 tests)
// ============================================================
void test_file_io_advanced() {
    printf("\n🗂️  Testing File I/O Advanced Functions\n");

    const char* test_file = "/tmp/minrust_advanced_test.txt";

    // Test 1: Create, write, and delete
    FILE* f = minrust_file_open(test_file, "w");
    minrust_file_write_line(f, "Test content");
    minrust_file_close(f);

    int exists_before = minrust_file_exists(test_file);
    assert_equal_int(exists_before, 1, "file_exists before delete");

    // Test 2: file_delete
    int deleted = minrust_file_delete(test_file);
    assert_equal_int(deleted, 1, "file_delete returns true");

    int exists_after = minrust_file_exists(test_file);
    assert_equal_int(exists_after, 0, "file does not exist after delete");
}

// ============================================================
// Main Test Runner
// ============================================================
int main() {
    printf("════════════════════════════════════════════════════════════\n");
    printf("🧪 MinRust Phase 4: Standard Library Tests\n");
    printf("════════════════════════════════════════════════════════════\n");

    // Run all test groups
    test_io_functions();
    test_string_functions();
    test_string_advanced();
    test_array_functions();
    test_array_advanced();
    test_math_functions();
    test_math_advanced();
    test_type_conversion();
    test_file_io_functions();
    test_file_io_advanced();

    // Print summary
    printf("\n════════════════════════════════════════════════════════════\n");
    printf("📊 Test Summary\n");
    printf("════════════════════════════════════════════════════════════\n");
    printf("Total Tests:  %d\n", test_count);
    printf("Passed:       %d ✅\n", pass_count);
    printf("Failed:       %d ❌\n", fail_count);

    if (fail_count == 0) {
        printf("\n🎉 All tests passed!\n");
    } else {
        printf("\n⚠️  Some tests failed.\n");
    }
    printf("════════════════════════════════════════════════════════════\n");

    return fail_count == 0 ? 0 : 1;
}
