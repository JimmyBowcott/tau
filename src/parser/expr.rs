use crate::{
    ast::{BinaryOp, Expr},
    token::Token,
};

use super::Parser;

impl Parser {
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Expr {
        let mut lhs = self.parse_primary();

        loop {
            let op = match self.peek() {
                Some(Token::Plus) => (1, 2, BinaryOp::Add),
                Some(Token::Minus) => (1, 2, BinaryOp::Subtract),
                Some(Token::Star) => (3, 4, BinaryOp::Multiply),
                Some(Token::Slash) => (3, 4, BinaryOp::Divide),
                Some(Token::Caret) => (5, 4, BinaryOp::Power),
                _ => break,
            };

            let (lbp, rbp, bop) = op;
            if lbp < min_bp {
                break;
            }

            self.advance();
            let rhs = self.parse_expr_bp(rbp);
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: bop,
                right: Box::new(rhs),
            };
        }

        lhs
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
}
