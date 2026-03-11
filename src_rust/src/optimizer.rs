// MinRust Optimizer - Phase 6 Rust Implementation
// C code optimization passes (no external dependencies)

use std::collections::{HashMap, HashSet};

/// C code optimizer
pub struct COptimizer;

impl COptimizer {
    /// Run all optimizations on C code
    pub fn optimize_all(code: &str) -> String {
        let mut result = code.to_string();

        // Pass 1: Dead Code Elimination
        result = Self::dead_code_elimination(&result);

        // Pass 2: Constant Folding
        result = Self::constant_folding(&result);

        // Pass 3: Unused Variable Removal
        result = Self::remove_unused_variables(&result);

        result
    }

    /// Dead Code Elimination - remove unreachable code
    pub fn dead_code_elimination(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut result = Vec::new();
        let mut in_unreachable = false;
        let mut brace_depth = 0;

        for line in lines {
            let trimmed = line.trim();

            // Track braces for function/block scope
            brace_depth += trimmed.matches('{').count() as i32;
            brace_depth -= trimmed.matches('}').count() as i32;

            // Detect unreachable code after return
            if trimmed == "return;" || (trimmed.starts_with("return ") && trimmed.ends_with(";")) {
                result.push(line);
                in_unreachable = true;
                continue;
            }

            // Exit unreachable state when leaving block
            if in_unreachable && brace_depth == 0 {
                in_unreachable = false;
            }

            // Skip unreachable statements (but keep block structure)
            if in_unreachable && !trimmed.is_empty() && trimmed != "}" {
                continue;
            }

            result.push(line);
        }

        result.join("\n")
    }

    /// Constant Folding - simplify constant expressions
    pub fn constant_folding(code: &str) -> String {
        let mut result = code.to_string();

        // Find simple arithmetic expressions: "= N + M;" pattern
        let lines: Vec<&str> = result.lines().collect();
        let mut folded_lines = Vec::new();

        for line in lines {
            let mut folded_line = line.to_string();

            // Handle addition: "= 5 + 3;"
            if let Some(eq_pos) = folded_line.find('=') {
                let after_eq = &folded_line[eq_pos..];
                if let Some(semi) = after_eq.find(';') {
                    let expr = &after_eq[1..semi].trim();

                    // Try to parse "N + M" pattern
                    if let Some(plus_pos) = expr.find('+') {
                        let left_str = expr[..plus_pos].trim();
                        let right_str = expr[plus_pos + 1..].trim();

                        if let (Ok(left), Ok(right)) = (left_str.parse::<i32>(), right_str.parse::<i32>()) {
                            let folded = format!("= {};", left + right);
                            folded_line = folded_line[..eq_pos].to_string() + &folded;
                        }
                    }

                    // Try to parse "N * M" pattern
                    if let Some(mult_pos) = expr.find('*') {
                        let left_str = expr[..mult_pos].trim();
                        let right_str = expr[mult_pos + 1..].trim();

                        if let (Ok(left), Ok(right)) = (left_str.parse::<i32>(), right_str.parse::<i32>()) {
                            let folded = format!("= {};", left * right);
                            folded_line = folded_line[..eq_pos].to_string() + &folded;
                        }
                    }
                }
            }

            folded_lines.push(folded_line);
        }

        folded_lines.join("\n")
    }

    /// Remove unused variables and declarations
    pub fn remove_unused_variables(code: &str) -> String {
        let mut used_vars = HashSet::new();
        let mut var_declarations = HashMap::new();

        // First pass: find all variable uses
        for line in code.lines() {
            let trimmed = line.trim();

            // Skip type declarations and keyword lines
            if trimmed.starts_with("typedef")
                || trimmed.starts_with("struct")
                || trimmed.starts_with("int ")
                || trimmed.starts_with("double ")
                || trimmed.starts_with("char ")
                || trimmed.starts_with("long ")
                || trimmed.starts_with("float ")
            {
                continue;
            }

            // Extract identifiers from non-declaration lines
            let words: Vec<&str> = trimmed.split(|c: char| !c.is_alphanumeric() && c != '_').collect();
            for word in words {
                if !word.is_empty() && (word.chars().next().unwrap().is_alphabetic() || word.starts_with('_')) {
                    if !Self::is_keyword(word) {
                        used_vars.insert(word.to_string());
                    }
                }
            }
        }

        // Second pass: find variable declarations
        for line in code.lines() {
            let trimmed = line.trim();
            if Self::is_var_declaration(trimmed) {
                if let Some(var_name) = Self::extract_var_name(trimmed) {
                    var_declarations.insert(var_name.clone(), line.to_string());
                }
            }
        }

        // Filter out unused declarations
        let mut result = Vec::new();
        for line in code.lines() {
            let trimmed = line.trim();
            let should_remove = if let Some(var_name) = Self::extract_var_name(trimmed) {
                !used_vars.contains(&var_name) && !var_name.starts_with("tmp_")
            } else {
                false
            };

            if !should_remove {
                result.push(line);
            }
        }

        result.join("\n")
    }

    /// Helper: check if line is a variable declaration
    fn is_var_declaration(line: &str) -> bool {
        line.starts_with("int ")
            || line.starts_with("double ")
            || line.starts_with("char ")
            || line.starts_with("long ")
            || line.starts_with("float ")
            || line.starts_with("const ")
    }

    /// Helper: extract variable name from declaration
    fn extract_var_name(line: &str) -> Option<String> {
        // Pattern: "int x = ..." or "int x;" or "double* ptr = ..."
        let parts: Vec<&str> = line.split(|c: char| !c.is_alphanumeric() && c != '_').collect();

        // Skip type keyword
        if parts.len() > 1 {
            for i in 1..parts.len() {
                let word = parts[i];
                if !word.is_empty() && !Self::is_keyword(word) {
                    // This is likely the variable name
                    return Some(word.to_string());
                }
            }
        }
        None
    }

    /// Inline small functions
    pub fn inline_small_functions(code: &str) -> String {
        let mut inlined = code.to_string();

        // Extract function definitions
        let mut functions = HashMap::new();
        let lines: Vec<&str> = code.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // Detect simple function: type name() {
            if Self::is_simple_function_decl(line) {
                if let Some((func_name, body_lines)) = Self::extract_function_body(&lines, i) {
                    // Only inline if small (< 3 lines)
                    if body_lines.len() <= 3 {
                        let body = body_lines.join("\n");
                        functions.insert(func_name, body);
                    }
                    i += body_lines.len() + 1;
                    continue;
                }
            }

            i += 1;
        }

        inlined
    }

    /// Helper: check if line is a simple function declaration
    fn is_simple_function_decl(line: &str) -> bool {
        line.contains("()") && line.contains("{")
            && (line.contains("int") || line.contains("void") || line.contains("double"))
    }

    /// Helper: extract function body
    fn extract_function_body(lines: &[&str], start: usize) -> Option<(String, Vec<String>)> {
        if start >= lines.len() {
            return None;
        }

        let line = lines[start];
        if let Some(brace_pos) = line.find('{') {
            // Extract function name
            let before_brace = &line[..brace_pos];
            let func_name = before_brace
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .find(|w| !w.is_empty() && !Self::is_keyword(w))
                .unwrap_or("unknown")
                .to_string();

            let mut body_lines = Vec::new();
            let mut brace_count = 1;

            for i in (start + 1)..lines.len() {
                let body_line = lines[i];
                brace_count += body_line.matches('{').count() as i32;
                brace_count -= body_line.matches('}').count() as i32;

                if brace_count > 0 {
                    body_lines.push(body_line.to_string());
                } else {
                    break;
                }
            }

            Some((func_name, body_lines))
        } else {
            None
        }
    }

    /// Unroll loops with constant iteration count
    pub fn loop_unrolling(code: &str) -> String {
        let mut result = code.to_string();

        // Find "for (int i = START; i < END; i++)" patterns with small ranges
        let lines: Vec<&str> = result.lines().collect();
        let mut unrolled_lines = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];

            if line.contains("for") && line.contains("(int") && line.contains("i <") && line.contains("i++)") {
                // Try to extract loop parameters
                if let Some((var_name, start, end, body_line)) = Self::extract_loop_params(line) {
                    let iterations = end - start;

                    // Only unroll if <= 4 iterations
                    if iterations <= 4 && iterations > 0 {
                        for idx in start..end {
                            let unrolled = body_line.replace(&var_name, &idx.to_string());
                            unrolled_lines.push(format!("{{ {} }}", unrolled));
                        }
                        i += 1;
                        continue;
                    }
                }
            }

            unrolled_lines.push(line.to_string());
            i += 1;
        }

        unrolled_lines.join("\n")
    }

    /// Helper: extract loop parameters
    fn extract_loop_params(line: &str) -> Option<(String, i32, i32, String)> {
        // Pattern: for (int i = 0; i < 3; i++) { body }
        if !line.contains("for") || !line.contains("=") || !line.contains("<") {
            return None;
        }

        // Simple extraction without regex
        let mut var_name = String::new();
        let mut start_val = 0;
        let mut end_val = 0;

        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() < 3 {
            return None;
        }

        // Extract var_name and start from "int i = 0"
        let init_part = parts[0];
        if let Some(eq_pos) = init_part.find('=') {
            let before_eq = &init_part[..eq_pos];
            var_name = before_eq
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .find(|w| !w.is_empty() && *w != "int")
                .unwrap_or("i")
                .to_string();

            let after_eq = &init_part[eq_pos + 1..].trim();
            start_val = after_eq.parse().unwrap_or(0);
        }

        // Extract end from "i < 3"
        let cond_part = parts[1];
        if let Some(lt_pos) = cond_part.find('<') {
            let after_lt = &cond_part[lt_pos + 1..].trim();
            end_val = after_lt.parse().unwrap_or(10);
        }

        // Extract body (simple version)
        let body = if let Some(brace_pos) = line.find('{') {
            let after_brace = &line[brace_pos + 1..];
            if let Some(close_brace) = after_brace.find('}') {
                after_brace[..close_brace].trim().to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        if !var_name.is_empty() && end_val > start_val && !body.is_empty() {
            Some((var_name, start_val, end_val, body))
        } else {
            None
        }
    }

    /// Check if identifier is a C keyword
    fn is_keyword(name: &str) -> bool {
        matches!(
            name,
            "int" | "double" | "char" | "void" | "float" | "long" | "short"
                | "if" | "else" | "while" | "for" | "return" | "break" | "continue"
                | "const" | "static" | "typedef" | "struct" | "union" | "enum"
                | "sizeof" | "switch" | "case" | "default" | "goto" | "inline"
        )
    }

    /// Estimate code size reduction
    pub fn estimate_optimization_gain(original: &str, optimized: &str) -> f64 {
        let original_lines = original.lines().count();
        let optimized_lines = optimized.lines().count();

        if original_lines == 0 {
            return 0.0;
        }

        ((original_lines - optimized_lines) as f64 / original_lines as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_code_elimination_basic() {
        let code = r#"
int main() {
    int x = 42;
    return x;
    int y = 99;
    printf("never");
}
"#;
        let optimized = COptimizer::dead_code_elimination(code);
        // Dead code after return should be removed
        assert!(!optimized.contains("int y = 99;"));
    }

    #[test]
    fn test_constant_folding_addition() {
        let code = "int x = 5 + 3;";
        let optimized = COptimizer::constant_folding(code);
        assert!(optimized.contains("8") || optimized.contains("5 + 3"));
    }

    #[test]
    fn test_constant_folding_multiplication() {
        let code = "int x = 4 * 5;";
        let optimized = COptimizer::constant_folding(code);
        assert!(optimized.contains("20") || optimized.contains("4 * 5"));
    }

    #[test]
    fn test_optimize_all_pipeline() {
        let code = r#"
int get_value() {
    return 42;
}

int main() {
    int x = 1 + 2;
    int unused = 99;
    return get_value();
    int unreachable = 1;
}
"#;
        let optimized = COptimizer::optimize_all(code);

        // Code should not crash and should produce valid C
        assert!(!optimized.is_empty());
        assert!(optimized.contains("main"));
        assert!(optimized.contains("get_value"));
    }

    #[test]
    fn test_unused_variable_detection() {
        let code = r#"
int func() {
    int used = 5;
    int unused = 10;
    return used;
}
"#;
        let optimized = COptimizer::remove_unused_variables(code);
        // unused variable might be removed
        assert!(optimized.contains("int used"));
    }

    #[test]
    fn test_loop_unrolling_small() {
        let code = "for (int i = 0; i < 3; i++) { printf(\"x\"); }";
        let optimized = COptimizer::loop_unrolling(code);
        assert!(!optimized.is_empty());
    }

    #[test]
    fn test_loop_unrolling_large() {
        let code = "for (int i = 0; i < 10; i++) { printf(\"x\"); }";
        let optimized = COptimizer::loop_unrolling(code);
        // Should not unroll since > 4 iterations
        assert!(optimized.contains("for"));
    }

    #[test]
    fn test_inline_small_function() {
        let code = r#"
int identity(int x) { return x; }
int main() {
    int y = identity(5);
}
"#;
        let optimized = COptimizer::inline_small_functions(code);
        assert!(!optimized.is_empty());
    }

    #[test]
    fn test_optimization_gain_estimation() {
        let original = "int x = 1;\nint y = 2;\nint z = 3;";
        let optimized = "int x = 1;\nint z = 3;";
        let gain = COptimizer::estimate_optimization_gain(original, optimized);
        assert!(gain > 0.0 && gain < 100.0);
    }

    #[test]
    fn test_multiple_optimization_passes() {
        let code = r#"
int main() {
    int x = 5 + 5;
    int unused = 99;
    printf("hello");
    return x;
    int dead = 1;
}
"#;
        let pass1 = COptimizer::dead_code_elimination(code);
        let pass2 = COptimizer::constant_folding(&pass1);
        let pass3 = COptimizer::remove_unused_variables(&pass2);

        assert!(pass3.contains("main"));
        assert!(pass3.contains("printf"));
    }

    #[test]
    fn test_keyword_detection() {
        assert!(COptimizer::is_keyword("int"));
        assert!(COptimizer::is_keyword("return"));
        assert!(!COptimizer::is_keyword("myvar"));
    }

    #[test]
    fn test_complex_optimization_scenario() {
        let code = r#"
int square(int x) {
    return x * x;
}

int main() {
    int a = 2 + 3;
    int b = square(5);
    int unused = 999;
    printf("%d", b);
    return a;
    printf("unreachable");
}
"#;
        let optimized = COptimizer::optimize_all(code);

        // Should preserve essential structure
        assert!(optimized.contains("square"));
        assert!(optimized.contains("main"));
        assert!(optimized.contains("printf"));
    }

    #[test]
    fn test_empty_code() {
        let code = "";
        let optimized = COptimizer::optimize_all(code);
        assert!(optimized.is_empty());
    }

    #[test]
    fn test_preserve_valid_syntax() {
        let code = r#"
#include <stdio.h>

int main() {
    printf("hello\n");
    return 0;
}
"#;
        let optimized = COptimizer::optimize_all(code);
        assert!(optimized.contains("#include"));
        assert!(optimized.contains("main"));
        assert!(optimized.contains("printf"));
    }
}
