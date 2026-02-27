

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
    IntLiteral(i32),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),

    Identifier(String),


    // Keywords
    Let,

    // Punctuation
    Semicolon,  // ;


    // Operators
    Equal,     // =
    EqualEqual, // ==
    Plus, // +
    Minus, // -
    Multiply, // *
    Divide, // /
    Modulus, // %
    LPRAN, // (
    RPRAN, // )
    PlusPlus, // ++
    MinusMinus, // --
    Power, // **



    // Others
    Eof,
}

impl TokenType{
    pub fn from_keyword(id : &str) -> Option<TokenType>{
        match id { 
            "let" => Some(TokenType::Let),
            _ => None
        }
    }

    pub fn precedence(&self) -> i32 {
        match self {
            TokenType::Plus | TokenType::Minus => 1,
            TokenType::Multiply | TokenType::Divide => 2,
            _ => 0,
        }
    }
}


/// Location of a token in a file
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub line   : usize,
    pub column : usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token{
    pub token : TokenType,
    pub location : Location
}