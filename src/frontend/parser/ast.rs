use crate::frontend::lexer::token::Location;
use std::fmt;

#[derive(Debug)]
pub enum Op {
    Add, Minus, Multiply, Divide,
}

#[derive(Debug)]
pub enum Expr {
    IntLiteral(i32),
    FloatLiteral(f64),
    Identifier(String),
    Binary(Box<Expr>, Op, Box<Expr>),
    StringLiteral(String),
    CharLiteral(char),
}


#[derive(Debug)]
pub struct LetStmt {
    pub name: String,
    pub value: Expr,
    pub location: Location,
}


#[derive(Debug)]
pub enum Stmt {
    Let(LetStmt),

}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::IntLiteral(n) => write!(f, "(int {})", n),
            Expr::FloatLiteral(fl) => write!(f, "(float {})", fl),
            Expr::Identifier(name) => write!(f, "(Ident {})", name),
            Expr::Binary(left, op, right) => write!(f, "({:?} {} {})", op, left, right),
            Expr::StringLiteral(name) => write!(f, "(Str \"{}\")", name),
            Expr::CharLiteral(name) => write!(f, "(Char \'{}\')", name),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Let(let_stmt) => {
                write!(f, "(let {} {})", let_stmt.name, let_stmt.value)
            }
        }
    }
}