use analysis::Analyser;
use error::Error;
use lexer::Lexer;
use parser::Parser;
use runtime::Env;

pub mod analysis;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod token;
pub mod error;

pub fn run_source(source: &str) -> Result<(), Error> {
    // TODO: Implement correct error types for analyser
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let mut analyser = Analyser::new();
    analyser.analyse(&stmts)?;

    let mut env = Env::new();
    for stmt in stmts {
        stmt.exec(&mut env)?;
    }

    Ok(())
}
