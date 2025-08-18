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
}
