use tau::{analysis::Analyser, lexer::Lexer, parser::Parser, runtime::Env};

fn run(source: &str) -> Result<(), String> {
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    let mut analyser = Analyser::new();
    analyser.analyse(&stmts)?;

    let mut env = Env::new();
    for stmt in stmts {
        stmt.exec(&mut env);
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let source = std::fs::read_to_string("test_file.tau")
        .map_err(|e| format!("Failed to read source file: {}", e))?;
    run(&source)?;
    Ok(())
}
