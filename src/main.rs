// src/main.rs

mod token;
mod lexer;
mod ast;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::io::{self, Write};

fn main() {
    let mut interpreter = Interpreter::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            break;
        }

        if trimmed_input.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(input.clone());
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == token::Token::EOF {
                break;
            }
            tokens.push(tok);
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let result = interpreter.interpret(&ast);
        println!("{}", result);
    }
}
