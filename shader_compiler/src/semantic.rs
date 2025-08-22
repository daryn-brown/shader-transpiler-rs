// src/semantic.rs
// This file contains the logic for semantic analysis.
// It walks the AST to check for correctness (e.g., type checking).

use crate::ast::*;
use std::collections::HashMap;

// A symbol table to keep track of declared variables and their types.
// This is a key part of semantic analysis.
type SymbolTable = HashMap<String, Type>;

// The main struct for our analyzer. It holds the symbol table.
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: HashMap::new(),
        }
    }

    // The main entry point for analysis. It takes the AST root.
    // The `&` indicates we are BORROWING the AST, not taking ownership.
    pub fn analyze(&mut self, ast: &FunctionDefinition) -> Result<(), String> {
        // First, add function parameters to the symbol table.
        for param in &ast.params {
            self.symbol_table
                .insert(param.name.clone(), param.type_name.clone());
        }

        // Then, walk through all statements in the function body.
        for statement in &ast.body {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    // This is the core of our AST "visitor". It uses pattern matching
    // to decide what to do for each kind of statement.
    fn visit_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Declaration {
                type_name,
                name,
                initializer,
            } => {
                // Check the expression and get its type
                let expr_type = self.visit_expression(initializer)?;

                // Add the new variable to the symbol table.
                println!("Declaring variable '{}' with type {:?}", name, type_name);
                self.symbol_table.insert(name.clone(), type_name.clone());
            }
            Statement::If {
                condition,
                if_block,
                ..
            } => {
                let condition_type = self.visit_expression(condition)?;
                for stmt in if_block {
                    self.visit_statement(stmt)?;
                }
            }
            Statement::Expression(expr) => {
                let _expr_type = self.visit_expression(expr)?;
            }
            Statement::Assert { condition } => {
                let condition_type = self.visit_expression(condition)?;
                if condition_type != Type::Bool {
                    return Err("Assert condition must be a bool".to_string());
                }
            }
        }
        Ok(())
    }

    fn visit_expression(&mut self, expression: &Expression) -> Result<Type, String> {
        match expression {
            Expression::Identifier(name) => {
                if let Some(t) = self.symbol_table.get(name) {
                    println!("Found usage of variable '{}'", name);
                    Ok(t.clone())
                } else {
                    Err(format!("Undeclared variable: {}", name))
                }
            }
            Expression::Assignment { name, value } => {
                if let Some(t) = self.symbol_table.get(name).cloned() {
                    let value_type = self.visit_expression(value)?;
                    // Optionally check t == value_type here
                    Ok(t)
                } else {
                    Err(format!("Undeclared variable in assignment: {}", name))
                }
            }
            Expression::BinaryOp { op, left, right } => {
                let left_type = self.visit_expression(left)?;
                let right_type = self.visit_expression(right)?;
                // Add type checking logic for operators here
                Ok(left_type) // or whatever is appropriate
            }
            Expression::IntLiteral(_) => Ok(Type::Int),
            Expression::FloatLiteral(_) => Ok(Type::Float),
            Expression::BoolLiteral(_) => Ok(Type::Bool),
        }
    }
}
