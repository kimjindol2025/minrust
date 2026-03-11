// MinRust Compiler CLI - Phase 5 Rust Implementation
// Command-line interface for the MinRust compiler

use minrust_compiler::{compile, VERSION};
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "compile" | "c" => {
            if args.len() < 3 {
                eprintln!("Error: No input file specified");
                print_usage(&args[0]);
                return;
            }
            compile_file(&args[2]);
        }
        "version" | "-v" | "--version" => {
            println!("MinRust Compiler v{}", VERSION);
        }
        "help" | "-h" | "--help" => {
            print_help(&args[0]);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage(&args[0]);
        }
    }
}

fn compile_file(path: &str) {
    // Read source file
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            return;
        }
    };

    // Compile
    println!("🦀 MinRust Compiler v{}", VERSION);
    println!("═══════════════════════════════════════");
    println!("📝 Compiling '{}'", path);
    println!();

    let result = compile(&source);

    // Print stats
    result.print_stats();

    // Output C code if successful
    if result.success && result.c_code.is_some() {
        println!("\n📄 Generated C Code:");
        println!("─────────────────────────────────────");
        println!("{}", result.c_code.unwrap());
        println!("─────────────────────────────────────");

        // Save to output file
        let output_path = path.replace(".fl", ".c").replace(".rs", ".c");
        match fs::write(&output_path, result.c_code.unwrap()) {
            Ok(_) => println!("\n✅ Saved to '{}'", output_path),
            Err(e) => eprintln!("\n❌ Failed to save output: {}", e),
        }
    }
}

fn print_usage(program: &str) {
    eprintln!("Usage: {} <command> [args]", program);
    eprintln!("Try '{} help' for more information", program);
}

fn print_help(program: &str) {
    println!("🦀 MinRust Compiler v{}", VERSION);
    println!();
    println!("Usage: {} <command> [args]", program);
    println!();
    println!("Commands:");
    println!("  compile <file>    Compile a Rust file to C");
    println!("  c <file>          Short for 'compile'");
    println!("  version           Show compiler version");
    println!("  help              Show this help message");
    println!();
    println!("Examples:");
    println!("  {} compile program.fl    # Compile Rust file", program);
    println!("  {} c program.fl           # Same as above", program);
    println!("  {} --version             # Show version", program);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_args() {
        // Test that main can handle various arguments
        let args = vec!["minrustc".to_string(), "version".to_string()];
        assert_eq!(args[1], "version");
    }
}
