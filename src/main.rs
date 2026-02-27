mod frontend;

use frontend::lexer::scanner::Lexer;
use frontend::parser::parse::Parser;

fn main() {
    let code : &str = "
        let z = 4.5;
        let x = (5 + (3*4) - z * (z+(7*8)));
    ";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    
    // for token in &tokens {
    //     println!("{:?}", token);
    // }
    

    let mut parser = Parser::new(tokens);
    let mut ast = parser.build_ast();

    for stmt in &ast {
        println!("{}", stmt); 
    }
    //println!("{:#?}", ast)
}
