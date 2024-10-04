// src/parser.rs

use crate::ast::ASTNode;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let first_token = tokens.get(0).cloned().unwrap_or(Token::EOF);
        Self {
            tokens,
            position: 0,
            current_token: first_token,
        }
    }

    fn advance(&mut self) {
        loop {
            self.position += 1;
            if self.position >= self.tokens.len() {
                self.current_token = Token::EOF;
                break;
            } else {
                self.current_token = self.tokens[self.position].clone();
                if self.current_token != Token::Newline {
                    break;
                }
            }
        }
    }

    pub fn parse(&mut self) -> ASTNode {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            if self.current_token == Token::Newline {
                self.advance();
                continue;
            }
            let stmt = self.parse_statement();
            statements.push(stmt);
        }
        ASTNode::Program(statements)
    }

    fn parse_statement(&mut self) -> ASTNode {
        if let Token::Identifier(name) = &self.current_token {
            let var_name = name.clone();
            self.advance();
            if let Token::Operator(op) = &self.current_token {
                if op == "=" {
                    self.advance();
                    let expr = self.parse_expression();
                    return ASTNode::Assignment {
                        name: var_name,
                        value: Box::new(expr),
                    };
                } else {
                    // Rückgängig machen, wenn kein '=' folgt
                    self.position -= 1;
                    self.current_token = Token::Identifier(var_name);
                }
            } else {
                // Rückgängig machen, wenn kein Operator folgt
                self.position -= 1;
                self.current_token = Token::Identifier(var_name);
            }
        }
        let expr = self.parse_expression();
        ASTNode::Statement(Box::new(expr))
    }

    fn parse_expression(&mut self) -> ASTNode {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();

        while let Token::Operator(op) = &self.current_token {
            if op == "+" || op == "-" {
                let operator = op.clone();
                self.advance();
                let right = self.parse_factor();
                node = ASTNode::BinaryOp {
                    left: Box::new(node),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        let mut node = self.parse_atom();

        while let Token::Operator(op) = &self.current_token {
            if op == "*" || op == "/" {
                let operator = op.clone();
                self.advance();
                let right = self.parse_atom();
                node = ASTNode::BinaryOp {
                    left: Box::new(node),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        node
    }

    fn parse_atom(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Number(value) => {
                let node = ASTNode::Number(*value);
                self.advance();
                node
            }
            Token::Identifier(name) => {
                let node = ASTNode::Variable(name.clone());
                self.advance();
                node
            }
            Token::Operator(op) if op == "(" => {
                self.advance();
                let expr = self.parse_expression();
                if let Token::Operator(closing) = &self.current_token {
                    if closing == ")" {
                        self.advance();
                        return expr;
                    } else {
                        panic!("Erwartete ')', gefunden: {:?}", self.current_token);
                    }
                } else {
                    panic!("Erwartete ')', gefunden: {:?}", self.current_token);
                }
            }
            _ => {
                panic!("Unerwartetes Token: {:?}", self.current_token);
            }
        }
    }
}
