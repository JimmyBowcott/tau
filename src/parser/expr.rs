use crate::{ast::{BinaryOp, Expr}, token::Token};

use super::Parser;

impl Parser {
    pub fn parse_expr(&mut self) -> Expr {
        let mut res = self.parse_primary();

        while let Some(op) = self.parse_operator() {
            res = Expr::Binary {
                left: Box::new(res),
                op,
                right: Box::new(self.parse_primary()),
            }
        }
        res
    }

    fn parse_primary(&mut self) -> Expr {
        let res = match self.advance() {
            Some(Token::Identifier(id)) => Expr::Identifier(id.clone()),
            Some(Token::Number(n)) => Expr::Number(n.clone()),
            Some(Token::LParen) => {
                let expr = self.parse_expr();
                if let Some(Token::RParen) = self.advance() {
                    expr
                } else {
                    panic!("Expected )");
                }
            }
            _ => panic!("Expected number or variable"),
        };
        res
    }

    fn parse_operator(&mut self) -> Option<BinaryOp> {
        if let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.advance();
                    return Some(BinaryOp::Add);
                }
                Token::Minus => {
                    self.advance();
                    return Some(BinaryOp::Subtract);
                }
                Token::Star => {
                    self.advance();
                    return Some(BinaryOp::Multiply);
                }
                Token::Slash => {
                    self.advance();
                    return Some(BinaryOp::Divide);
                }
                _ => return None,
            }
        } else {
            panic!("Expected ;")
        }
    }
}
