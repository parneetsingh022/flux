mod frontend;

use frontend::lexer::scanner::Lexer;
use frontend::parser::parse::Parser;

fn main() {
    let code : &str = "

    ";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    
    for token in &tokens {
        println!("{:?}", token);
    }
    

    // let mut parser = Parser::new(tokens);
    // let mut ast = parser.build_ast();

    // for stmt in &ast {
    //     println!("{}", stmt); 
    // }
}
