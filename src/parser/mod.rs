pub mod expr;
pub mod stmt;
pub mod unit;

use crate::ast::Stmt;
use crate::error::Error;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt()? {
                self.expect_token(TokenKind::Semicolon, "Expected ';' after statement")?;
                stmts.push(stmt);
            } else {
                break;
            }
        }
        Ok(stmts)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        self.line = tok.line;
        self.column = tok.column;
        Some(tok)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
