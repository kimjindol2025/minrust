// MinRust Standard Library - C Header
// Phase 4: Core standard functions for MinRust runtime

#ifndef MINRUST_STDLIB_H
#define MINRUST_STDLIB_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <ctype.h>

// ============================================================
// I/O Functions (6 functions)
// ============================================================

// println(text: &str)
void minrust_println(const char* text) {
    printf("%s\n", text);
}

// print(text: &str)
void minrust_print(const char* text) {
    printf("%s", text);
}

// input() -> String
char* minrust_input() {
    char* buffer = (char*)malloc(1024);
    if (fgets(buffer, 1024, stdin) != NULL) {
        // Remove trailing newline
        size_t len = strlen(buffer);
        if (len > 0 && buffer[len-1] == '\n') {
            buffer[len-1] = '\0';
        }
    }
    return buffer;
}

// dbg(value: i32) -> i32
int minrust_dbg(int value) {
    printf("[DEBUG] %d\n", value);
    return value;
}

// eprintln(text: &str)
void minrust_eprintln(const char* text) {
    fprintf(stderr, "%s\n", text);
}

// eprint(text: &str)
void minrust_eprint(const char* text) {
    fprintf(stderr, "%s", text);
}

// ============================================================
// String Functions (12 functions)
// ============================================================

// strlen_custom(s: &str) -> usize
int minrust_strlen(const char* s) {
    return (int)strlen(s);
}

// strcat_custom(s1: &str, s2: &str) -> String
char* minrust_strcat(const char* s1, const char* s2) {
    char* result = (char*)malloc(strlen(s1) + strlen(s2) + 1);
    strcpy(result, s1);
    strcat(result, s2);
    return result;
}

// strslice(s: &str, start: usize, end: usize) -> String
char* minrust_strslice(const char* s, int start, int end) {
    int len = (int)strlen(s);
    if (start < 0) start = 0;
    if (end > len) end = len;
    if (start >= end) {
        char* empty = (char*)malloc(1);
        empty[0] = '\0';
        return empty;
    }

    int slice_len = end - start;
    char* result = (char*)malloc(slice_len + 1);
    strncpy(result, s + start, slice_len);
    result[slice_len] = '\0';
    return result;
}

// substr(s: &str, index: usize, length: usize) -> String
char* minrust_substr(const char* s, int index, int length) {
    return minrust_strslice(s, index, index + length);
}

// to_uppercase(s: &str) -> String
char* minrust_to_uppercase(const char* s) {
    char* result = (char*)malloc(strlen(s) + 1);
    strcpy(result, s);
    for (int i = 0; result[i]; i++) {
        result[i] = toupper(result[i]);
    }
    return result;
}

// to_lowercase(s: &str) -> String
char* minrust_to_lowercase(const char* s) {
    char* result = (char*)malloc(strlen(s) + 1);
    strcpy(result, s);
    for (int i = 0; result[i]; i++) {
        result[i] = tolower(result[i]);
    }
    return result;
}

// trim(s: &str) -> String
char* minrust_trim(const char* s) {
    while (*s && isspace(*s)) s++;
    int len = (int)strlen(s);
    while (len > 0 && isspace(s[len-1])) len--;

    char* result = (char*)malloc(len + 1);
    strncpy(result, s, len);
    result[len] = '\0';
    return result;
}

// contains(s: &str, substr: &str) -> bool
int minrust_contains(const char* s, const char* substr) {
    return strstr(s, substr) != NULL ? 1 : 0;
}

// starts_with(s: &str, prefix: &str) -> bool
int minrust_starts_with(const char* s, const char* prefix) {
    return strncmp(s, prefix, strlen(prefix)) == 0 ? 1 : 0;
}

// ends_with(s: &str, suffix: &str) -> bool
int minrust_ends_with(const char* s, const char* suffix) {
    int s_len = (int)strlen(s);
    int suffix_len = (int)strlen(suffix);
    if (suffix_len > s_len) return 0;
    return strcmp(s + s_len - suffix_len, suffix) == 0 ? 1 : 0;
}

// find(s: &str, pattern: &str) -> i32
int minrust_find(const char* s, const char* pattern) {
    char* pos = strstr(s, pattern);
    if (pos == NULL) return -1;
    return (int)(pos - s);
}

// replace(s: &str, from: &str, to: &str) -> String
char* minrust_replace(const char* s, const char* from, const char* to) {
    char* result = (char*)malloc(strlen(s) * 2 + 1);
    result[0] = '\0';

    const char* ptr = s;
    int from_len = (int)strlen(from);

    while (*ptr) {
        if (strncmp(ptr, from, from_len) == 0) {
            strcat(result, to);
            ptr += from_len;
        } else {
            strncat(result, ptr, 1);
            ptr++;
        }
    }
    return result;
}

// ============================================================
// Array/Vector Functions (10 functions)
// ============================================================

// Array structure for dynamic arrays
typedef struct {
    int* data;
    int len;
    int capacity;
} IntArray;

// array_new() -> Array
IntArray minrust_array_new() {
    IntArray arr;
    arr.capacity = 10;
    arr.len = 0;
    arr.data = (int*)malloc(arr.capacity * sizeof(int));
    return arr;
}

// array_push(arr: &mut Array, value: i32)
void minrust_array_push(IntArray* arr, int value) {
    if (arr->len >= arr->capacity) {
        arr->capacity *= 2;
        arr->data = (int*)realloc(arr->data, arr->capacity * sizeof(int));
    }
    arr->data[arr->len++] = value;
}

// array_pop(arr: &mut Array) -> i32
int minrust_array_pop(IntArray* arr) {
    if (arr->len > 0) {
        return arr->data[--arr->len];
    }
    return 0;
}

// array_len(arr: &Array) -> usize
int minrust_array_len(IntArray* arr) {
    return arr->len;
}

// array_get(arr: &Array, index: usize) -> i32
int minrust_array_get(IntArray* arr, int index) {
    if (index >= 0 && index < arr->len) {
        return arr->data[index];
    }
    return 0;
}

// array_set(arr: &mut Array, index: usize, value: i32)
void minrust_array_set(IntArray* arr, int index, int value) {
    if (index >= 0 && index < arr->len) {
        arr->data[index] = value;
    }
}

// array_clear(arr: &mut Array)
void minrust_array_clear(IntArray* arr) {
    arr->len = 0;
}

// array_contains(arr: &Array, value: i32) -> bool
int minrust_array_contains(IntArray* arr, int value) {
    for (int i = 0; i < arr->len; i++) {
        if (arr->data[i] == value) return 1;
    }
    return 0;
}

// array_index_of(arr: &Array, value: i32) -> i32
int minrust_array_index_of(IntArray* arr, int value) {
    for (int i = 0; i < arr->len; i++) {
        if (arr->data[i] == value) return i;
    }
    return -1;
}

// array_free(arr: &mut Array)
void minrust_array_free(IntArray* arr) {
    free(arr->data);
    arr->len = 0;
    arr->capacity = 0;
}

// ============================================================
// Math Functions (12 functions)
// ============================================================

// abs(x: i32) -> i32
int minrust_abs(int x) {
    return x < 0 ? -x : x;
}

// abs_f64(x: f64) -> f64
double minrust_abs_f64(double x) {
    return fabs(x);
}

// sqrt(x: f64) -> f64
double minrust_sqrt(double x) {
    return sqrt(x);
}

// pow(base: f64, exp: f64) -> f64
double minrust_pow(double base, double exp) {
    return pow(base, exp);
}

// min(a: i32, b: i32) -> i32
int minrust_min(int a, int b) {
    return a < b ? a : b;
}

// max(a: i32, b: i32) -> i32
int minrust_max(int a, int b) {
    return a > b ? a : b;
}

// floor(x: f64) -> f64
double minrust_floor(double x) {
    return floor(x);
}

// ceil(x: f64) -> f64
double minrust_ceil(double x) {
    return ceil(x);
}

// round(x: f64) -> f64
double minrust_round(double x) {
    return round(x);
}

// sin(x: f64) -> f64
double minrust_sin(double x) {
    return sin(x);
}

// cos(x: f64) -> f64
double minrust_cos(double x) {
    return cos(x);
}

// tan(x: f64) -> f64
double minrust_tan(double x) {
    return tan(x);
}

// ============================================================
// Type Conversion Functions (6 functions)
// ============================================================

// to_string(value: i32) -> String
char* minrust_int_to_string(int value) {
    char* result = (char*)malloc(20);
    sprintf(result, "%d", value);
    return result;
}

// to_string_f64(value: f64) -> String
char* minrust_float_to_string(double value) {
    char* result = (char*)malloc(30);
    sprintf(result, "%f", value);
    return result;
}

// parse_int(s: &str) -> i32
int minrust_parse_int(const char* s) {
    return atoi(s);
}

// parse_float(s: &str) -> f64
double minrust_parse_float(const char* s) {
    return atof(s);
}

// to_bool(s: &str) -> bool
int minrust_to_bool(const char* s) {
    if (strcmp(s, "true") == 0 || strcmp(s, "1") == 0) return 1;
    return 0;
}

// to_char(value: i32) -> char
char minrust_to_char(int value) {
    return (char)value;
}

// ============================================================
// File I/O Functions (6 functions)
// ============================================================

// file_open(path: &str, mode: &str) -> FILE*
FILE* minrust_file_open(const char* path, const char* mode) {
    return fopen(path, mode);
}

// file_close(file: FILE*)
void minrust_file_close(FILE* file) {
    if (file != NULL) {
        fclose(file);
    }
}

// file_read_line(file: FILE*) -> String
char* minrust_file_read_line(FILE* file) {
    if (file == NULL) return NULL;

    char* line = (char*)malloc(1024);
    if (fgets(line, 1024, file) != NULL) {
        size_t len = strlen(line);
        if (len > 0 && line[len-1] == '\n') {
            line[len-1] = '\0';
        }
        return line;
    }
    free(line);
    return NULL;
}

// file_write_line(file: FILE*, text: &str)
void minrust_file_write_line(FILE* file, const char* text) {
    if (file != NULL) {
        fprintf(file, "%s\n", text);
    }
}

// file_exists(path: &str) -> bool
int minrust_file_exists(const char* path) {
    FILE* file = fopen(path, "r");
    if (file != NULL) {
        fclose(file);
        return 1;
    }
    return 0;
}

// file_delete(path: &str) -> bool
int minrust_file_delete(const char* path) {
    return remove(path) == 0 ? 1 : 0;
}

#endif // MINRUST_STDLIB_H
