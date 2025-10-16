use tau::{analysis::Analyser, lexer::Lexer, parser::Parser, runtime::Env };

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
    run("let t = 5; let velocity: m/s = 0.5 * t^2 + 10 / (2 * t); print(velocity * 2);")?;
    Ok(())
}
