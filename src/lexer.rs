// src/lexer.rs

use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    indent_stack: Vec<usize>, // Stack zur Verfolgung der Einrückungsebenen
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars = input.chars().collect::<Vec<_>>();
        let first_char = chars.get(0).cloned();
        Self {
            input: chars,
            position: 0,
            current_char: first_char,
            indent_stack: vec![0], // Startet mit Einrückungsebene 0
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

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
    }

    pub fn next_token(&mut self) -> Token {
        // Behandle Einrückungen und Ausrückungen
        if self.current_char == Some('\n') {
            self.advance();

            let mut num_spaces = 0;
            while let Some(c) = self.current_char {
                if c == ' ' {
                    num_spaces += 1;
                    self.advance();
                } else if c == '\n' {
                    // Leere Zeile, num_spaces zurücksetzen
                    num_spaces = 0;
                    self.advance();
                } else {
                    break;
                }
            }

            let current_indent = *self.indent_stack.last().unwrap();

            if num_spaces > current_indent {
                self.indent_stack.push(num_spaces);
                return Token::Indent;
            } else if num_spaces < current_indent {
                self.indent_stack.pop();
                return Token::Dedent;
            } else {
                return Token::Newline;
            }
        }

        // Überspringe Leerzeichen (außer Newlines)
        while let Some(c) = self.current_char {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }

        // Überprüfe auf EOF
        if self.current_char.is_none() {
            // Verarbeite verbleibende Dedents am EOF
            if self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                return Token::Dedent;
            }
            return Token::EOF;
        }

        let c = self.current_char.unwrap();

        // Behandle Kommentare
        if c == '#' {
            while let Some(c) = self.current_char {
                if c == '\n' {
                    break;
                }
                self.advance();
            }
            return self.next_token();
        }

        // Behandle Newline
        if c == '\n' {
            // Sollte bereits verarbeitet sein
            self.advance();
            return Token::Newline;
        }

        // Behandle Zeichenketten
        if c == '"' {
            return self.string_literal();
        }

        // Behandle Zahlen
        if c.is_digit(10) {
            return self.number();
        }

        // Behandle Bezeichner und Schlüsselwörter
        if c.is_alphabetic() || c == '_' {
            return self.identifier();
        }

        // Behandle mehrstellige Operatoren
        if c == '=' {
            if let Some('=') = self.peek_char() {
                self.advance();
                self.advance();
                return Token::Operator("==".to_string());
            } else {
                self.advance();
                return Token::Assign;
            }
        }

        if c == '!' {
            if let Some('=') = self.peek_char() {
                self.advance();
                self.advance();
                return Token::Operator("!=".to_string());
            } else {
                self.advance();
                return Token::Not;
            }
        }

        if c == '<' || c == '>' {
            if let Some('=') = self.peek_char() {
                let op = format!("{}=", c);
                self.advance();
                self.advance();
                return Token::Operator(op);
            } else {
                self.advance();
                return Token::Operator(c.to_string());
            }
        }

        // Behandle einfache Operatoren und Satzzeichen
        match c {
            '+' | '-' | '*' | '/' | '%' => {
                self.advance();
                Token::Operator(c.to_string())
            }
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            ':' => {
                self.advance();
                Token::Colon
            }
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

        match result.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "def" => Token::Def,
            "return" => Token::Return,
            "print" => Token::Print,
            "True" => Token::True,
            "False" => Token::False,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Identifier(result),
        }
    }

    fn number(&mut self) -> Token {
        let mut result = String::new();
        let mut has_decimal_point = false;
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else if c == '.' && !has_decimal_point {
                has_decimal_point = true;
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(result.parse::<f64>().unwrap())
    }

    fn string_literal(&mut self) -> Token {
        self.advance(); // Überspringe das öffnende Anführungszeichen
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // Überspringe das schließende Anführungszeichen
                break;
            } else {
                result.push(c);
                self.advance();
            }
        }
        Token::StringLiteral(result)
    }
}
