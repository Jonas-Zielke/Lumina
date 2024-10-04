// src/ast.rs

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(Box<ASTNode>),
    Number(f64),
    Variable(String),
    BinaryOp {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    // Weitere Knoten können hier hinzugefügt werden
}
