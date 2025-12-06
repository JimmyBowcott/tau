use std::process;

use tau::{
    analysis::Analyser, error::Error, lexer::Lexer, parser::Parser, runtime::Env,
    source::SourceFile,
};

fn compile_and_execute(source: &str) -> Result<(), Error> {
    // TODO: Implement correct error types for these other two...
    let lexer = Lexer::new(source);
    let tokens = lexer.collect();

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse().map_err(|e| Error::new(1, 1, e))?;

    let mut analyser = Analyser::new();
    analyser.analyse(&stmts).map_err(|e| Error::new(1, 1, e))?;

    let mut env = Env::new();
    for stmt in stmts {
        stmt.exec(&mut env)?;
    }
    Ok(())
}

fn run() -> Result<(), Error> {
    use std::env;
    let args: Vec<String> = env::args().collect();

    // TODO: Check for exactly one argument!
    let (name, text) = if args.len() > 1 {
        let path = &args[1];
        let text = std::fs::read_to_string(path).map_err(|e| Error::io(&args[1], e))?;
        (path.clone(), text)
    } else {
        use std::io::{self, Read};
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| Error::io("<stdin>", e))?;
        ("<stdin>".to_string(), buffer)
    };

    let source = SourceFile::new(name.clone(), text);
    compile_and_execute(&source.text)
        .map_err(|e| e.with_source(name.as_str(), source.get_line(e.line)))?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
