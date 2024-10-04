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
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 1 {
        // Dateipfad wurde übergeben
        let file_path = &args[1];
        let code = fs::read_to_string(file_path)
            .unwrap_or_else(|_| panic!("Konnte Datei '{}' nicht lesen", file_path));
        execute_code(&code, &mut interpreter);
    } else {
        // Interaktiver Modus
        repl(&mut interpreter);
    }
}

fn execute_code(code: &str, interpreter: &mut Interpreter) {
    let mut lexer = Lexer::new(code.to_string());
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

    interpreter.interpret(&ast);
}

fn repl(interpreter: &mut Interpreter) {
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

        execute_code(&input, interpreter);
    }
}
