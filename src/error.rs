use std::fmt;

use crate::ast::Expr;

pub struct Error {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub span: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pointer = "^".repeat(self.span);
        write!(
            f,
            "Error at {}:{}\n{}\n{}{}",
            self.line,
            self.column,
            self.message,
            " ".repeat(self.column - 1),
            pointer
        )
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl Expr {
    pub fn error(&self, message: String) -> Error {
        Error {
            message,
            line: self.line,
            column: self.column,
            span: 1,
        }
    }
}
