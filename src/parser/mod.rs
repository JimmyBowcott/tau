pub mod expr;
pub mod stmt;
pub mod unit;

use crate::ast::Stmt;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                self.expect_token(Token::Semicolon, "Expected ';' after statement");
                stmts.push(stmt);
            } else {
                break;
            }
        }
        stmts
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
        Some(tok)
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
