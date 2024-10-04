// src/interpreter.rs

use crate::ast::ASTNode;
use std::collections::HashMap;

pub struct Interpreter {
    environment: Environment,
}

type Environment = HashMap<String, Value>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Function {
        params: Vec<String>,
        body: Box<ASTNode>,
    },
    ReturnValue(Box<Value>),
    Null,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, node: &ASTNode) -> Value {
        match node {
            ASTNode::Program(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.interpret(stmt);
                    if let Value::ReturnValue(_) = result {
                        return result;
                    }
                }
                result
            }
            ASTNode::Block(statements) => {
                let mut result = Value::Null;
                let local_env = self.environment.clone();
                for stmt in statements {
                    result = self.interpret(stmt);
                    if let Value::ReturnValue(_) = result {
                        return result;
                    }
                }
                self.environment = local_env;
                result
            }
            ASTNode::Statement(expr) => self.interpret(expr),
            ASTNode::Number(value) => Value::Number(*value),
            ASTNode::String(value) => Value::String(value.clone()),
            ASTNode::Boolean(value) => Value::Boolean(*value),
            ASTNode::Variable(name) => self.environment.get(name).cloned().unwrap_or(Value::Null),
            ASTNode::Assignment { name, value } => {
                let val = self.interpret(value);
                self.environment.insert(name.clone(), val.clone());
                val
            }
            ASTNode::BinaryOp { left, operator, right } => {
                let left_val = self.interpret(left);
                let right_val = self.interpret(right);
                self.evaluate_binary_op(&left_val, operator, &right_val)
            }
            ASTNode::UnaryOp { operator, operand } => {
                let val = self.interpret(operand);
                self.evaluate_unary_op(operator, &val)
            }
            ASTNode::IfStatement { condition, then_branch, else_branch } => {
                let cond_value = self.interpret(condition);
                if self.is_truthy(&cond_value) {
                    self.interpret(then_branch)
                } else if let Some(else_node) = else_branch {
                    self.interpret(else_node)
                } else {
                    Value::Null
                }
            }
            ASTNode::WhileLoop { condition, body } => {
                let mut result = Value::Null;
                loop {
                    let cond_value = self.interpret(condition);
                    if !self.is_truthy(&cond_value) {
                        break;
                    }
                    result = self.interpret(body);
                    if let Value::ReturnValue(_) = result {
                        return result;
                    }
                }
                result
            }
            ASTNode::FunctionDef { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: Box::new(*body.clone()),
                };
                self.environment.insert(name.clone(), func.clone());
                func
            }
            ASTNode::FunctionCall { name, args } => {
                let func = self.environment.get(name).cloned();
                if let Some(Value::Function { params, body }) = func {
                    if params.len() != args.len() {
                        panic!("Falsche Anzahl von Argumenten für Funktion '{}'", name);
                    }
                    let mut new_env = self.environment.clone();
                    for (param, arg) in params.iter().zip(args) {
                        let val = self.interpret(arg);
                        new_env.insert(param.clone(), val);
                    }
                    let mut interpreter = Interpreter {
                        environment: new_env,
                    };
                    let result = interpreter.interpret(&body);
                    if let Value::ReturnValue(val) = result {
                        *val
                    } else {
                        Value::Null
                    }
                } else {
                    panic!("Funktion '{}' nicht definiert", name);
                }
            }
            ASTNode::Return(expr) => {
                let val = self.interpret(expr);
                Value::ReturnValue(Box::new(val))
            }
            ASTNode::Print(expr) => {
                let val = self.interpret(expr);
                println!("{}", self.value_to_string(&val));
                Value::Null
            }
        }
    }

    fn evaluate_binary_op(&self, left: &Value, operator: &str, right: &Value) -> Value {
        match operator {
            "+" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Number(l + r)
                } else if let (Value::String(l), Value::String(r)) = (left, right) {
                    Value::String(l.clone() + r)
                } else {
                    panic!("Ungültiger Operandentyp für '+': {:?} und {:?}", left, right);
                }
            }
            "-" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Number(l - r)
                } else {
                    panic!("Ungültiger Operandentyp für '-': {:?} und {:?}", left, right);
                }
            }
            "*" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Number(l * r)
                } else {
                    panic!("Ungültiger Operandentyp für '*': {:?} und {:?}", left, right);
                }
            }
            "/" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    if *r == 0.0 {
                        panic!("Division durch Null");
                    }
                    Value::Number(l / r)
                } else {
                    panic!("Ungültiger Operandentyp für '/': {:?} und {:?}", left, right);
                }
            }
            "%" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Number(l % r)
                } else {
                    panic!("Ungültiger Operandentyp für '%': {:?} und {:?}", left, right);
                }
            }
            "==" => Value::Boolean(left == right),
            "!=" => Value::Boolean(left != right),
            "<" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Boolean(l < r)
                } else {
                    panic!("Ungültiger Operandentyp für '<': {:?} und {:?}", left, right);
                }
            }
            ">" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Boolean(l > r)
                } else {
                    panic!("Ungültiger Operandentyp für '>': {:?} und {:?}", left, right);
                }
            }
            "<=" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Boolean(l <= r)
                } else {
                    panic!("Ungültiger Operandentyp für '<=': {:?} und {:?}", left, right);
                }
            }
            ">=" => {
                if let (Value::Number(l), Value::Number(r)) = (left, right) {
                    Value::Boolean(l >= r)
                } else {
                    panic!("Ungültiger Operandentyp für '>=': {:?} und {:?}", left, right);
                }
            }
            "and" => {
                Value::Boolean(self.is_truthy(left) && self.is_truthy(right))
            }
            "or" => {
                Value::Boolean(self.is_truthy(left) || self.is_truthy(right))
            }
            _ => panic!("Unbekannter Operator: {}", operator),
        }
    }

    fn evaluate_unary_op(&self, operator: &str, operand: &Value) -> Value {
        match operator {
            "-" => {
                if let Value::Number(v) = operand {
                    Value::Number(-v)
                } else {
                    panic!("Ungültiger Operandentyp für '-': {:?}", operand);
                }
            }
            "+" => {
                if let Value::Number(v) = operand {
                    Value::Number(*v)
                } else {
                    panic!("Ungültiger Operandentyp für '+': {:?}", operand);
                }
            }
            "not" => {
                Value::Boolean(!self.is_truthy(operand))
            }
            _ => panic!("Unbekannter Operator: {}", operator),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            _ => true,
        }
    }

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Function { .. } => "<function>".to_string(),
            Value::ReturnValue(val) => self.value_to_string(val),
        }
    }
}
