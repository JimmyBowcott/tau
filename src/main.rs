use tau::{analysis::Analyser, error::Error, lexer::Lexer, parser::Parser, runtime::Env};

fn run(source: &str) -> Result<(), Error> {
    // TODO: Implement correct error types for these other two...
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().map_err(|e| Error {
        message: e,
        line: 0,
        column: 0,
        span: 0,
    })?;

    let mut analyser = Analyser::new();
    analyser.analyse(&stmts).map_err(|e| Error {
        message: e,
        line: 0,
        column: 0,
        span: 0,
    })?;

    let mut env = Env::new();
    for stmt in stmts {
        stmt.exec(&mut env)?;
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    // TODO: Fix this
    let source = std::fs::read_to_string("test_file.tau")
        .map_err(|e| Error {
                message: format!("Failed to read source file: {}", e),
                line: 0,
                column: 0,
                span: 0,
            })?;

    run(&source)?;
    Ok(())
}
