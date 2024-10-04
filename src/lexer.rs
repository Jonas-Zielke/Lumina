// src/lexer.rs

use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        let first_char = chars.get(0).cloned();
        Self {
            input: chars,
            position: 0,
            current_char: first_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
    }

    pub fn next_token(&mut self) -> Token {
        // Überspringe Leerzeichen und Tabs
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }

        // Überprüfe auf EOF
        if self.current_char.is_none() {
            return Token::EOF;
        }

        let c = self.current_char.unwrap();

        // Behandle verschiedene Token-Typen
        if c.is_alphabetic() || c == '_' {
            return self.identifier();
        }

        if c.is_digit(10) {
            return self.number();
        }

        // Behandle Newline
        if c == '\n' {
            self.advance();
            return Token::Newline;
        }

        // Behandle Operatoren und Sonderzeichen
        self.advance();
        match c {
            '+' | '-' | '*' | '/' | '=' | '(' | ')' => Token::Operator(c.to_string()),
            _ => {
                panic!("Unbekanntes Zeichen: {}", c);
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Identifier(result)
    }

    fn number(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(result.parse::<f64>().unwrap())
    }
}
