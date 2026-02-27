use crate::frontend::lexer::token::{Token, TokenType, Location};
use crate::frontend::parser::ast::{
    Expr,
    LetStmt,
    Stmt
};

pub struct Parser{
    tokens : Vec<Token>,
    pos : usize,
    ast : Vec<Stmt>,
}



impl Parser{
    pub fn new(tokens : Vec<Token>) -> Self{
        Self{
            tokens,
            pos: 0,
            ast: vec![],
        }
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.pos);
    }

    fn next(&mut self) -> Option<&Token> {
        let result = self.tokens.get(self.pos);

        self.pos += 1;
        
        return result;
    }

    fn consume(&mut self, expected: TokenType, msg: &str) -> Token {
        match self.next() {
            Some(token) if std::mem::discriminant(&token.token) == std::mem::discriminant(&expected) => token.clone(),
            Some(token) => panic!("{} Found {:?} at line {}, col {}", msg, token.token, token.location.line, token.location.column),
            None => panic!("{} but reached end of file", msg),
        }
    }

    pub fn build_ast(&mut self) -> Vec<Stmt>{
        while let Some(token) = self.peek() {
            match &token.token {
                TokenType::Eof => break,
                TokenType::Let => {
                    let location = token.location.clone();
                    self.next();
                    self.parse_let_stmt(location);
                },
                _ => {
                    let location = token.location.clone();
                    panic!("Unexpected token {:?} at line {}, column {}.", token.token, location.line, location.column);
                }
            }
        }

        std::mem::take(&mut self.ast)
    }

    fn parse_let_stmt(&mut self, location : Location)  {
        // Expect an identifier
        let id_token = self.consume(TokenType::Identifier(String::new()), "Expected identifier after 'let'");
        
        // Extract the string (since it's inside the enum)
        let name = match id_token.token {
            TokenType::Identifier(s) => s,
            _ => unreachable!(),
        };

        // Expect '='
        self.consume(TokenType::Equal, "Expected '=' after identifier");

        // Expect a value
        let value = self.parse_expression();

        // 5. Expect ';'
        self.consume(TokenType::Semicolon, "Expected ';' after statement");

        self.ast.push(Stmt::Let(LetStmt { name, value , location}));
    }

    fn parse_expression(&mut self) -> Expr {
        let token = self.next().expect("Expected expression");
        match &token.token {
            TokenType::IntLiteral(n) => Expr::IntLiteral(*n),
            TokenType::FloatLiteral(f) => Expr::FloatLiteral(*f),
            TokenType::Identifier(name) => Expr::Identifier(name.clone()),
            _ => panic!("Expected expression at line {}, column {}.", token.location.line, token.location.column),
        }
    }
}


