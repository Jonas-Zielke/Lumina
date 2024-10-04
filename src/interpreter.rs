// src/interpreter.rs

use crate::ast::ASTNode;
use std::collections::HashMap;

pub struct Interpreter {
    environment: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, node: &ASTNode) -> f64 {
        match node {
            ASTNode::Program(statements) => {
                let mut result = 0.0;
                for stmt in statements {
                    result = self.interpret(stmt);
                }
                result
            }
            ASTNode::Statement(expr) => self.interpret(expr),
            ASTNode::Number(value) => *value,
            ASTNode::BinaryOp { left, operator, right } => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                match operator.as_str() {
                    "+" => left_val + right_val,
                    "-" => left_val - right_val,
                    "*" => left_val * right_val,
                    "/" => left_val / right_val,
                    _ => panic!("Unbekannter Operator: {}", operator),
                }
            }
            ASTNode::Variable(name) => {
                *self.environment.get(name).expect(&format!(
                    "Variable '{}' wurde nicht definiert",
                    name
                ))
            }
            ASTNode::Assignment { name, value } => {
                let val = self.interpret(value);
                self.environment.insert(name.clone(), val);
                val
            }
        }
    }
}
