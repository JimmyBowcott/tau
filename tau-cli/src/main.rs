use std::process;

use tau_cli::source::SourceFile;
use tau_core::{error::Error, run_source};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let (name, text) = get_input()?;

    let source = SourceFile::new(name.clone(), text);
    run_source(&source.text)
        .map_err(|e| e.with_source(name.as_str(), source.get_line(e.line)))?;
    Ok(())
}

fn get_input() -> Result<(String, String), Error> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
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
