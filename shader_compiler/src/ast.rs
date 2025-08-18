// src/ast.rs
// This file defines the Rust structs and enums that form our
// Abstract Syntax Tree (AST). This structure represents the code's
// hierarchy and meaning, decoupled from the raw source text.

// Using Debug trait to allow printing the AST for inspection.
#[derive(Debug, PartialEq)]
pub enum Type {
    Float,
    Int,
    Bool,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub return_type: Type,
    pub name: String,
    pub params: Vec<Parameter>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub type_name: Type,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration {
        type_name: Type,
        name: String,
        initializer: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        if_block: Vec<Statement>,
        // else_block can be added later
    },
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    BinaryOp {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
