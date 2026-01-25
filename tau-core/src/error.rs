use std::fmt;

use crate::ast::Expr;

pub struct Error {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub span: usize,
    pub filename: String,
    pub line_text: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pointer = "^".repeat(self.span);
        write!(
            f,
            "Error in {}:{}:{}\n\n{}\n{}{} {}",
            self.filename,
            self.line,
            self.column,
            self.line_text,
            " ".repeat(self.column - 1),
            pointer,
            self.message,
        )
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Debug error: {}",
            self.message,
        )
    }
}

impl Error {
    pub fn new(line: usize, column: usize, message: String) -> Self {
        Self {
            message,
            line,
            column,
            span: 1,
            filename: String::new(),
            line_text: String::new(),
        }
    }
    pub fn io(path: &str, err: std::io::Error) -> Self {
        Self {
            message: format!("Failed to read {}: {}", path, err),
            line: 0,
            column: 0,
            span: 0,
            filename: String::new(),
            line_text: String::new(),
        }
    }

    pub fn with_source(&self, filename: &str, text: &str) -> Self {
        Self {
            message: self.message.clone(),
            line: self.line,
            column: self.column,
            span: self.span,
            filename: String::from(filename),
            line_text: String::from(text),
        }
    }
}

impl Expr {
    pub fn error(&self, message: String) -> Error {
        Error {
            message,
            line: self.line,
            column: self.column,
            span: 1,
            filename: String::new(),
            line_text: String::new(),
        }
    }
}
