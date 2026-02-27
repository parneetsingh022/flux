
use crate::frontend::lexer::token::{TokenType, Token, Location};

pub struct Lexer{
    input : Vec<u8>,
    pos : usize,
    line: usize,
    column : usize,
    tokens : Vec<Token>
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.bytes().collect(),
            pos: 0,
            line: 1,
            column: 1,
            tokens : vec![]
        }
    }

    fn peek(&self) -> Option<u8> {
        return self.input.get(self.pos).copied();
    }

    fn next(&mut self) -> Option<u8> {
        let result = self.input.get(self.pos).copied();

        if let Some(byte) = result{
            self.pos += 1;
            if byte == b'\n'{
                self.column = 1;
                self.line += 1;
            }else{
                self.column += 1;
            }
        }
        
        return result;
    }

    fn add_token(&mut self, token_type : TokenType, line : usize, column : usize){
        self.tokens.push(
            Token{
                token: token_type, 
                location: Location{line, column}
            }
        );
    }

    pub fn tokenize(&mut self) -> Vec<Token>{
        while let Some(cur) = self.peek() {
            let start_line = self.line;
            let start_col = self.column;
            self.next();
            match cur {
                b' ' | b'\r' | b'\t' | b'\n' => continue,
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_identifier(cur, start_line, start_col),
                b'0'..=b'9' => self.read_number(cur, start_line, start_col),
                b'=' => self.read_equal(start_line, start_col),
                b';' => self.add_token(TokenType::Semicolon, start_line, start_col),
                b'+' | b'-' | b'*' | b'/' | b'%' | b'(' | b')'   => self.read_operator(cur,  start_line, start_col),
                b'\"' => self.read_string(start_line, start_col),
                b'\'' => self.read_char(start_line, start_col),
                _ => panic!(
                    "Lexer Error: Found unknown character '{}' at line {}, col {}.", 
                    cur as char, self.line, self.column
                ),
            }
        }
        
        self.add_token(TokenType::Eof, self.line, self.column);
        self.tokens.clone()
    }

    pub fn read_identifier(&mut self, cur : u8, start_line : usize, start_column : usize){
        let mut identifier = String::new();
        identifier.push(cur as char);

        while let Some(next_char) = self.peek() {
            if next_char.is_ascii_alphanumeric() || next_char == b'_' {
                identifier.push(self.next().unwrap() as char);
            } else {
                break;
            }
        }

        let token_type = TokenType::from_keyword(&identifier)
            .unwrap_or(TokenType::Identifier(identifier));

        self.add_token(token_type, start_line, start_column);
    }

    pub fn read_number(&mut self, cur : u8, start_line : usize, start_column : usize){
        let mut isFloat = false;
        let mut number = String::new();
        number.push(cur as char);

        while let Some(next_char) = self.peek() {
            if next_char.is_ascii_digit() {
                number.push(self.next().unwrap() as char);
            } else if next_char == b'.' && !isFloat{
                isFloat = true;
                number.push(self.next().unwrap() as char);
            }else {
                break;
            }
        }
        
        if !isFloat{
            let value = number.parse::<i32>().unwrap();
            self.add_token(TokenType::IntLiteral(value), start_line, start_column);
            return;
        }

        let value = number.parse::<f64>().unwrap();
        self.add_token(TokenType::FloatLiteral(value), start_line, start_column);
    }

    pub fn read_equal(&mut self, start_line : usize, start_column : usize){
        let next = self.peek();
        if next == Some(b'=') {
            self.add_token(TokenType::EqualEqual, start_line, start_column);
            self.next();
        }else{
            self.add_token(TokenType::Equal, start_line, start_column);
        }
    }

    pub fn read_operator(&mut self, cur : u8, start_line : usize, start_column : usize){
        let next = self.peek();
        match cur {
            b'+' => {
                if next == Some(b'+') {
                    self.next(); // Consume the second '+'
                    self.add_token(TokenType::PlusPlus, start_line, start_column);
                } else {
                    self.add_token(TokenType::Plus, start_line, start_column);
                }
            },
            b'-' => {
                if next == Some(b'-') {
                    self.next(); // Consume the second '-'
                    self.add_token(TokenType::MinusMinus, start_line, start_column);
                } else {
                    self.add_token(TokenType::Minus, start_line, start_column);
                }
            }
            b'*' => {
                if next == Some(b'*') {
                    self.next(); // Consume the second '-'
                    self.add_token(TokenType::Power, start_line, start_column);
                } else {
                    self.add_token(TokenType::Multiply, start_line, start_column);
                }
            }
            b'/' => self.add_token(TokenType::Divide, start_line, start_column),
            b'%' => self.add_token(TokenType::Modulus, start_line, start_column),
            b'(' => self.add_token(TokenType::LPRAN, start_line, start_column),
            b')' => self.add_token(TokenType::RPRAN, start_line, start_column),
            _ => panic!("Unexpected operator '{}', at line {}, column {}.", cur as char, start_line, start_column)
        }
    }

    pub fn read_string(&mut self, start_line: usize, start_column: usize) {
        let mut string_content = String::new();

        while let Some(next_char) = self.next() {
            if next_char == b'\"' {
                // Found the closing quote, add the token and return
                self.add_token(TokenType::StringLiteral(string_content), start_line, start_column);
                return;
            }
            
            // Add character to string
            string_content.push(next_char as char);
        }

        // If the loop finishes without returning, the string was never closed
        panic!(
            "Lexer Error: String never terminated at line {}, col {}, forgot to add '\"'.", 
            start_line, start_column
        );
    }

    pub fn read_char(&mut self, start_line: usize, start_column: usize) {
        let mut content = String::new();

        while let Some(next_char) = self.peek() {
            if next_char == b'\'' {
                self.next(); // Consume the closing quote
                break;
            }
            if next_char == b'\n' {
                break; // Stop at newline to prevent scanning the whole file
            }
            content.push(self.next().unwrap() as char);
        }

        if content.len() == 1 {
            // exactly one character
            let c = content.chars().next().unwrap();
            self.add_token(TokenType::CharLiteral(c), start_line, start_column);
        } else if content.is_empty() {
            panic!("Lexer Error: Empty character literal at line {}, col {}", start_line, start_column);
        } else {
            // Error case: show the full string found inside the single quotes
            panic!(
                "Lexer Error: Character literal can only contain one character. Found '{}' at line {}, col {}", 
                content, start_line, start_column
            );
        }
    }
}