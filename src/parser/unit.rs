use crate::{ast::{UnitExpr, UnitOp}, token::Token};

use super::Parser;

impl Parser {
    pub fn parse_unit_expr(&mut self) -> UnitExpr {
        let mut node = self.parse_unit();

        while let Some(token) = self.peek() {
            let op;
            match token {
                Token::Dot => {
                    self.advance();
                    op = UnitOp::Multiply;
                }
                Token::Slash => {
                    self.advance();
                    op = UnitOp::Divide;
                }
                _ => break,
            }
            let right = self.parse_unit();
            node = UnitExpr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }
        node
    }

    fn parse_unit(&mut self) -> UnitExpr {
        let base = self.parse_unit_base();
        if let Some(exponent) = self.parse_unit_exponent() {
            return UnitExpr::Power {
                base: Box::new(base),
                exponent,
            };
        }
        base
    }

    fn parse_unit_base(&mut self) -> UnitExpr {
        let base = if let Some(Token::Identifier(id)) = self.advance() {
            UnitExpr::Symbol(id.clone())
        } else {
            panic!("Expected unit identifier");
        };
        base
    }

    fn parse_unit_exponent(&mut self) -> Option<f64> {
        if let Some(Token::Caret) = self.peek() {
            self.advance();
            if let Some(Token::Number(n)) = self.advance() {
                return Some(n.clone());
            } else {
                panic!("Expected number after ^");
            }
        } else {
            return None;
        }
    }
}
