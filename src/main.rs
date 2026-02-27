mod frontend;

use frontend::lexer::scanner::Lexer;
use frontend::parser::parse::Parser;

fn main() {
    let code : &str = "
    let x = 29;
    let k = x;
    ";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    // lexer.print_tokens();

    let mut parser = Parser::new(tokens);
    let mut ast = parser.build_ast();

    for stmt in &ast {
        println!("{}", stmt); 
    }
}
