use crate::{
    ast::{Expr, Stmt, UnitExpr},
    token::Token,
};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek()? {
            Token::Let => self.parse_let_stmt(),
            Token::Print => self.parse_print_stmt(),
            _ => None,
        }
    }

    pub fn expect_token(&mut self, expected: Token, err_msg: &str) {
        if self.advance() != Some(&expected) {
            panic!("{}", err_msg);
        }
    }

    fn expect_identifier(&mut self, err_msg: &str) -> String {
        if let Some(Token::Identifier(id)) = self.advance() {
            id.clone()
        } else {
            panic!("{}", err_msg);
        }
    }

    fn expect_unit(&mut self, err_msg: &str) -> UnitExpr {
        self.advance();

        if let Some(Token::Identifier(_)) = self.peek() {
           self.parse_unit_expr()
        } else {
            panic!("{}", err_msg);
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.advance();

        let name = self.expect_identifier("Expected identifier after 'let'");

        let mut unit = None;

        if let Some(Token::Colon) = self.peek() {
            unit = Some(self.expect_unit("Expected unit after ':'"));
        }

        self.expect_token(Token::Equal, "Expected expression after '='");
        let value = self.parse_expr();

        Some(Stmt::Let { name, unit, value })
    }

    fn parse_print_stmt(&mut self) -> Option<Stmt> {
        self.advance();
        let expr: Expr;

        if let Some(Token::Identifier(name)) = self.peek() {
            expr = Expr::Identifier(name.clone());
            self.advance();
        } else {
            expr = self.parse_expr();
        }

        Some(Stmt::Print(expr))
    }
}
