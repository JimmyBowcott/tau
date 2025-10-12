use lexer::Lexer;
use parser::Parser;
use runtime::Env;

mod ast;
mod lexer;
mod parser;
mod token;
mod runtime;

fn run(source: &str) {
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    let mut env = Env::new();
    for stmt in stmts {
        stmt.exec(&mut env);
    }
}

fn main() {
    run("let t = 5; let velocity: m/s = 0.5 * t^2 + 10 / (2 * t); print(velocity * 2);");
}
