use lexer::{Lexer, Token};
use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let lexer = Lexer::new("let velocity: m/s = 10.0 / (2 * t);");
    let tokens: Vec<Token> = lexer.collect();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("{:#?}", ast);
}
