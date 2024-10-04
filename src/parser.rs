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

    fn expect(&mut self, expected: Token) {
        if self.current_token == expected {
            self.advance();
        } else {
            panic!(
                "Erwartetes Token: {:?}, Gefunden: {:?}",
                expected, self.current_token
            );
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
        match &self.current_token {
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_loop(),
            Token::Def => self.parse_function_def(),
            Token::Return => self.parse_return_statement(),
            Token::Print => self.parse_print_statement(),
            _ => {
                if let Token::Identifier(_) = &self.current_token {
                    if self.peek_token() == Token::Assign {
                        self.parse_assignment()
                    } else {
                        self.parse_expression_statement()
                    }
                } else {
                    self.parse_expression_statement()
                }
            }
        }
    }

    fn parse_if_statement(&mut self) -> ASTNode {
        self.advance(); // 'if'

        let condition = self.parse_expression();

        self.expect(Token::Colon);

        let then_branch = self.parse_block();

        let else_branch = if self.current_token == Token::Else {
            self.advance(); // 'else'
            self.expect(Token::Colon);
            let else_branch = self.parse_block();
            Some(Box::new(else_branch))
        } else {
            None
        };

        ASTNode::IfStatement {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        }
    }

    fn parse_while_loop(&mut self) -> ASTNode {
        self.advance(); // 'while'

        let condition = self.parse_expression();

        self.expect(Token::Colon);

        let body = self.parse_block();

        ASTNode::WhileLoop {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    fn parse_function_def(&mut self) -> ASTNode {
        self.advance(); // 'def'

        let name = if let Token::Identifier(name) = &self.current_token {
            name.clone()
        } else {
            panic!("Funktionsname erwartet");
        };
        self.advance();

        self.expect(Token::LeftParen);

        let mut params = Vec::new();
        if self.current_token != Token::RightParen {
            loop {
                if let Token::Identifier(name) = &self.current_token {
                    params.push(name.clone());
                    self.advance();
                } else {
                    panic!("Parametername erwartet");
                }
                if self.current_token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(Token::RightParen);

        self.expect(Token::Colon);

        let body = self.parse_block();

        ASTNode::FunctionDef {
            name,
            params,
            body: Box::new(body),
        }
    }

    fn parse_return_statement(&mut self) -> ASTNode {
        self.advance(); // 'return'
        let value = self.parse_expression();
        ASTNode::Return(Box::new(value))
    }

    fn parse_print_statement(&mut self) -> ASTNode {
        self.advance(); // 'print'
        let value = self.parse_expression();
        ASTNode::Print(Box::new(value))
    }

    fn parse_assignment(&mut self) -> ASTNode {
        if let Token::Identifier(name) = &self.current_token {
            let var_name = name.clone();
            self.advance();
            self.expect(Token::Assign);
            let expr = self.parse_expression();
            ASTNode::Assignment {
                name: var_name,
                value: Box::new(expr),
            }
        } else {
            panic!("Variable erwartet");
        }
    }

    fn parse_expression_statement(&mut self) -> ASTNode {
        let expr = self.parse_expression();
        ASTNode::Statement(Box::new(expr))
    }

    fn parse_block(&mut self) -> ASTNode {
    if self.current_token == Token::Newline {
        self.advance(); // Newline überspringen
        self.expect(Token::Indent);

        let mut statements = Vec::new();
        while self.current_token != Token::Dedent && self.current_token != Token::EOF {
            if self.current_token == Token::Newline {
                self.advance();
                continue;
            }
            let stmt = self.parse_statement();
            statements.push(stmt);
        }

        self.expect(Token::Dedent);

        ASTNode::Block(statements)
    } else {
        // Einzeiler nach dem ':'
        let stmt = self.parse_statement();
        ASTNode::Block(vec![stmt])
    }
}


    fn parse_expression(&mut self) -> ASTNode {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> ASTNode {
        let mut node = self.parse_logic_and();

        while self.current_token == Token::Or {
            self.advance();
            let right = self.parse_logic_and();
            node = ASTNode::BinaryOp {
                left: Box::new(node),
                operator: "or".to_string(),
                right: Box::new(right),
            };
        }

        node
    }

    fn parse_logic_and(&mut self) -> ASTNode {
        let mut node = self.parse_equality();

        while self.current_token == Token::And {
            self.advance();
            let right = self.parse_equality();
            node = ASTNode::BinaryOp {
                left: Box::new(node),
                operator: "and".to_string(),
                right: Box::new(right),
            };
        }

        node
    }

    fn parse_equality(&mut self) -> ASTNode {
        let mut node = self.parse_comparison();

        while let Token::Operator(op) = &self.current_token {
            if op == "==" || op == "!=" {
                let operator = op.clone();
                self.advance();
                let right = self.parse_comparison();
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

    fn parse_comparison(&mut self) -> ASTNode {
        let mut node = self.parse_term();

        while let Token::Operator(op) = &self.current_token {
            if ["<", ">", "<=", ">="].contains(&op.as_str()) {
                let operator = op.clone();
                self.advance();
                let right = self.parse_term();
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
        let mut node = self.parse_unary();

        while let Token::Operator(op) = &self.current_token {
            if op == "*" || op == "/" || op == "%" {
                let operator = op.clone();
                self.advance();
                let right = self.parse_unary();
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

    fn parse_unary(&mut self) -> ASTNode {
        if let Token::Operator(op) = &self.current_token {
            if op == "-" || op == "+" {
                let operator = op.clone();
                self.advance();
                let operand = self.parse_unary();
                return ASTNode::UnaryOp {
                    operator,
                    operand: Box::new(operand),
                };
            }
        } else if self.current_token == Token::Not {
            let operator = "not".to_string();
            self.advance();
            let operand = self.parse_unary();
            return ASTNode::UnaryOp {
                operator,
                operand: Box::new(operand),
            };
        }
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Number(value) => {
                let node = ASTNode::Number(*value);
                self.advance();
                node
            }
            Token::StringLiteral(value) => {
                let node = ASTNode::String(value.clone());
                self.advance();
                node
            }
            Token::True => {
                self.advance();
                ASTNode::Boolean(true)
            }
            Token::False => {
                self.advance();
                ASTNode::Boolean(false)
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                if self.current_token == Token::LeftParen {
                    // Funktionsaufruf
                    self.advance(); // '('
                    let mut args = Vec::new();
                    if self.current_token != Token::RightParen {
                        loop {
                            let arg = self.parse_expression();
                            args.push(arg);
                            if self.current_token == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RightParen);
                    ASTNode::FunctionCall {
                        name: var_name,
                        args,
                    }
                } else {
                    ASTNode::Variable(var_name)
                }
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(Token::RightParen);
                expr
            }
            _ => {
                panic!("Unerwartetes Token in parse_atom: {:?}", self.current_token);
            }
        }
    }

    fn peek_token(&self) -> Token {
        if self.position + 1 >= self.tokens.len() {
            Token::EOF
        } else {
            self.tokens[self.position + 1].clone()
        }
    }
}
