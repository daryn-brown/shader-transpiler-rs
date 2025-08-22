Oxide Shader Transpiler (shaderrs)
<p align="center">
<img src="https://img.shields.io/badge/language-Rust-orange.svg" alt="Language: Rust"/>
<img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"/>
<img src="https://img.shields.io/badge/build-passing-brightgreen.svg" alt="Build: Passing"/>
</p>

A next-generation shader compiler and toolchain, built in Rust, designed to transpile a custom, ergonomic shading language into multiple high-performance backends like GLSL and SPIR-V.

💡 Core Philosophy
Writing shaders can often feel like a black box. You write code, compile it, and see the visual output. When something goes wrong, debugging is a painful process of changing code and guessing at the results.

This project aims to change that.

Our core goal is to create a shading language and compiler with debuggability and developer experience as first-class citizens. We believe that by providing better tools, richer static analysis, and innovative debugging features directly in the language, we can make graphics programming faster, more intuitive, and more powerful.

✨ The Oxide Shading Language (OSL)
shaderrs compiles OSL, a language designed to be familiar to anyone who has written HLSL or GLSL, but with modern features and safety guarantees inspired by Rust.

Current Features (v0.1.0)
Primitive Types: float, int, bool

Variable Declarations: Statically-typed variable declarations and assignments.

Control Flow: if statements for conditional execution.

Expressions: Full support for arithmetic (+, -, *, /) and comparison (==, !=, <, >) operators.

Compiler Frontend:

Parsing: A robust parser built with LALRPOP that generates a complete Abstract Syntax Tree (AST).

Semantic Analysis: A validation layer that performs:

Declaration Checking: Ensures all variables are declared before use.

Type Checking: Enforces strict type compatibility in assignments, operations, and control flow.

🗺️ Project Roadmap
This project is under active development. Here is a high-level overview of our planned features:

[x] Phase 1: Compiler Front-End

[x] Lexer & Parser (LALRPOP)

[x] Abstract Syntax Tree (AST) Generation

[x] Semantic Analysis (Variable & Type Checking)

[ ] Phase 2: Language Feature Expansion

[ ] More types (float2, float3, float4, matrices)

[ ] return statements

[ ] Loops (for, while)

[ ] Function calls

[ ] Phase 3: Debuggability & Tooling

[ ] assert(condition) for runtime validation.

[ ] debug_log(...) for printf-style value inspection on the GPU.

[ ] Trace-based, single-pixel step-through debugging simulation.

[ ] Phase 4: Compiler Back-End

[ ] Intermediate Representation (IR) design.

[ ] GLSL code generator.

[ ] SPIR-V binary generator.

🚀 Getting Started
Prerequisites
Rust toolchain and Cargo.

Build & Run
Clone the repository:

git clone https://github.com/your-username/shader-transpiler-rs.git
cd shader-transpiler-rs/shader_compiler

Run the compiler:

cargo run

The program will parse and analyze the sample shader code located in src/main.rs. Feel free to modify the source_code string in main.rs to experiment with the compiler's current capabilities and error reporting.

Technology Stack
Language: Rust

Parsing: LALRPOP Parser Generator