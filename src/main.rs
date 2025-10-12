use ast::Stmt;
use lexer::Lexer;
use parser::Parser;
use token::Token;

mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    let lexer = Lexer::new("let velocity: m/s = 0.5 * t^2 + 10 / (2 * t); let v = 1;");
    let tokens: Vec<Token> = lexer.collect();
    let mut parser = Parser::new(tokens);
    let ast: Vec<Stmt> = parser.parse();
    println!("{:#?}", ast);
}
