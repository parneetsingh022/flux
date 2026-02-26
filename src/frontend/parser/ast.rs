use crate::frontend::lexer::token::Location;

#[derive(Debug)]
pub enum Op {
    Add, Subtract, Multiply, Divide,
}

#[derive(Debug)]
pub enum Expr {
    IntLiteral(i32),
    FloatLiteral(f64),
    Variable(String),
    Binary(Box<Expr>, Op, Box<Expr>),
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