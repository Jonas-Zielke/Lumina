// src/ast.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(Box<ASTNode>),
    Block(Vec<ASTNode>), // Hinzugefügt: Block-Knoten
    Number(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    BinaryOp {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    UnaryOp {
        operator: String,
        operand: Box<ASTNode>,
    },
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    WhileLoop {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
    Return(Box<ASTNode>),
    Print(Box<ASTNode>),
    // Weitere Knoten können hier hinzugefügt werden
}
