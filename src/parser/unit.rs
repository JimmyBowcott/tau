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
                },
                Token::Slash => {
                    self.advance();
                    op = UnitOp::Divide;
                },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::ast::{UnitExpr, UnitOp};

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_single_unit() {
        let tokens = vec![Token::Identifier("m".into())];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr();
        assert_eq!(expr, UnitExpr::Symbol("m".into()));
    }

    #[test]
    fn parse_unit_with_exponent() {
        let tokens = vec![Token::Identifier("s".into()), Token::Caret, Token::Number(2.0)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr();
        assert_eq!(
            expr,
            UnitExpr::Power {
                base: Box::new(UnitExpr::Symbol("s".into())),
                exponent: 2.0,
            }
        );
    }

    #[test]
    fn parse_composite_unit_multiply() {
        let tokens = vec![
            Token::Identifier("kg".into()),
            Token::Dot,
            Token::Identifier("m".into()),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr();
        assert_eq!(
            expr,
            UnitExpr::Binary {
                left: Box::new(UnitExpr::Symbol("kg".into())),
                op: UnitOp::Multiply,
                right: Box::new(UnitExpr::Symbol("m".into())),
            }
        );
    }

    #[test]
    fn parse_composite_unit_divide() {
        let tokens = vec![
            Token::Identifier("m".into()),
            Token::Slash,
            Token::Identifier("s".into()),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr();
        assert_eq!(
            expr,
            UnitExpr::Binary {
                left: Box::new(UnitExpr::Symbol("m".into())),
                op: UnitOp::Divide,
                right: Box::new(UnitExpr::Symbol("s".into())),
            }
        );
    }

    #[test]
    fn parse_complex_unit() {
        // kg.m/s^2
        let tokens = vec![
            Token::Identifier("kg".into()),
            Token::Dot,
            Token::Identifier("m".into()),
            Token::Slash,
            Token::Identifier("s".into()),
            Token::Caret,
            Token::Number(2.0),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr();

        let expected = UnitExpr::Binary {
            left: Box::new(UnitExpr::Binary {
                left: Box::new(UnitExpr::Symbol("kg".into())),
                op: UnitOp::Multiply,
                right: Box::new(UnitExpr::Symbol("m".into())),
            }),
            op: UnitOp::Divide,
            right: Box::new(UnitExpr::Power {
                base: Box::new(UnitExpr::Symbol("s".into())),
                exponent: 2.0,
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    #[should_panic(expected = "Expected unit identifier")]
    fn parse_empty_unit_should_panic() {
        let tokens = vec![];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }

    #[test]
    #[should_panic(expected = "Expected number after ^")]
    fn parse_unit_missing_exponent_should_panic() {
        let tokens = vec![Token::Identifier("s".into()), Token::Caret];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }

    #[test]
    #[should_panic(expected = "Expected number after ^")]
    fn parse_unit_after_exponent_should_panic() {
        let tokens = vec![Token::Identifier("s".into()), Token::Caret, Token::Identifier("m".into())];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }

    #[test]
    #[should_panic(expected = "Expected number after ^")]
    fn parse_symbol_after_exponent_should_panic() {
        let tokens = vec![Token::Identifier("s".into()), Token::Caret, Token::Caret];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }
}
