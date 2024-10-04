// src/token.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Operator(String),
    If,
    Else,
    While,
    Def,
    Return,
    Print,
    True,
    False,
    And,
    Or,
    Not,
    Assign,
    LeftParen,
    RightParen,
    Comma,
    Colon,
    Newline,
    Indent,   // Hinzugefügt: Indent-Token
    Dedent,   // Hinzugefügt: Dedent-Token
    EOF,
}
