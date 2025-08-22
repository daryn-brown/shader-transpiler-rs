// src/main.rs
// This is the main entry point for our compiler application.

// Use a module to hold the AST definitions.
pub mod ast;
pub mod semantic;

// This line includes the Rust code that LALRPOP generates from our
// shader.lalrpop grammar file. The `#[allow]` attributes are to
// prevent warnings from unused code that lalrpop might generate.
#[allow(clippy::all)]
#[allow(dead_code)]
pub mod shader {
    include!(concat!(env!("OUT_DIR"), "/shader.rs"));
}

fn main() {
    println!("--- Shader Compiler ---");

    let source_code = r#"
        float my_func(int a) {
            float c = 1.0;
            if (a > 10) {
               c = 2.0;
            }
        }
    "#;

    println!("\nParsing source code:\n{}", source_code);

    // Instantiate our generated parser.
    let parser = shader::TranslationUnitParser::new();

    // Parse the source code.
    match parser.parse(source_code) {
        Ok(parsed_ast) => {
            println!("\nSuccessfully parsed into AST!");
            // The `:#?` format specifier pretty-prints the debug output.
            println!("{:#?}", parsed_ast);

            println!("\n--- Phase 2: Semantic Analysis ---");
            // Create an instance of our new analyzer.
            let mut analyzer = semantic::SemanticAnalyzer::new();

            // Run the analysis.
            match analyzer.analyze(&parsed_ast) {
                Ok(_) => {
                    println!("\nSemantic analysis passed!");
                }
                Err(e) => {
                    println!("\nSemantic analysis failed!");
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("\nFailed to parse!");
            println!("{}", e);
        }
    }

    let test_cases = [
        (
            "Valid assert",
            r#"
            float my_func(int a) {
                float c = 1.0;
                assert(a > 0);
            }
            "#
        ),
        (
            "Assert with type error",
            r#"
            float my_func(int a) {
                float c = 1.0;
                assert(c); 
            }
            "#
        ),
        (
            "Undeclared variable",
            r#"
            float my_func(int a) {
                assert(b > 0); 
            }
            "#
        ),
        (
            "Assert with boolean literal",
            r#"
            float my_func(int a) {
                assert(true);
            }
            "#
        ),
        (
            "Assignment type mismatch",
            r#"
            float my_func(int a) {
                int x = 5;
                x = 3.14;
            }
            "#
        ),
        (
            "Redeclaration of variable",
            r#"
            float my_func(int a) {
                int x = 1;
                int x = 2;
            }
            "#
        ),
        (
            "If condition not boolean",
            r#"
            float my_func(int a) {
                if (a) {
                    int x = 1;
                }
            }
            "#
        ),
        (
            "Use before declaration",
            r#"
            float my_func(int a) {
                x = 5;
                int x = 1;
            }
            "#
        ),
    ];

    for (desc, src) in test_cases {
        println!("\n--- Test: {desc} ---");
        let parser = shader::TranslationUnitParser::new();
        match parser.parse(src) {
            Ok(ast) => {
                let mut analyzer = semantic::SemanticAnalyzer::new();
                match analyzer.analyze(&ast) {
                    Ok(_) => println!("Semantic analysis passed!"),
                    Err(e) => println!("Semantic analysis failed: {e}"),
                }
            }
            Err(e) => println!("Parse failed: {e}"),
        }
    }
}
