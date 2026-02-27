mod frontend;

use frontend::lexer::scanner::Lexer;
use frontend::parser::parse::Parser;

fn main() {
    let code : &str = "
        let z = (\"HEllo world\" + 'd') - 25.6;
    ";

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    
    // for token in &tokens {
    //     println!("{:?}", token);
    // }
    

    let mut parser = Parser::new(tokens);
    let ast = parser.build_ast();

    for stmt in &ast {
        println!("{}", stmt); 
    }
    //println!("{:#?}", ast)
}
