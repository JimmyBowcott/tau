use std::process;

use tau::{
    analysis::Analyser, error::Error, lexer::Lexer, parser::Parser, runtime::Env,
    source::SourceFile,
};

fn compile_and_execute(source: &str) -> Result<(), Error> {
    // TODO: Implement correct error types for analyser
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let mut analyser = Analyser::new();
    analyser.analyse(&stmts).map_err(|e| Error::new(1, 1, e))?;

    let mut env = Env::new();
    for stmt in stmts {
        stmt.exec(&mut env)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let (name, text) = get_input()?;

    let source = SourceFile::new(name.clone(), text);
    compile_and_execute(&source.text)
        .map_err(|e| e.with_source(name.as_str(), source.get_line(e.line)))?;
    Ok(())
}

fn get_input() -> Result<(String, String), Error> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => read_stdin().map(|t| ("<stdin>".to_string(), t)),
        2 => {
            let path = args[1].clone();
            read_file(&path).map(|t| (path, t))
        }
        _ => Err(Error::new(1, 1, format!("Usage: {} [file.tau]", args[0]))),
    }
}

fn read_file(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path).map_err(|e| Error::io(path, e))
}

fn read_stdin() -> Result<String, Error> {
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| Error::io("<stdin>", e))?;
    Ok(buffer)
}
