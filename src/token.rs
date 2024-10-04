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
    LeftBracket,    // Hinzugefügt: LeftBracket für Listen
    RightBracket,   // Hinzugefügt: RightBracket für Listen
    Comma,          // Bereits vorhanden, kann für Listen und Tupel genutzt werden
    Colon,
    Newline,
    Indent,
    Dedent,
    EOF,
}
