// src/token.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Operator(String),
    Newline,
    EOF,
    // Wenn du später weitere Tokens benötigst, kannst du sie hier hinzufügen
}
