use crate::{
    ast::{UnitExpr, UnitOp},
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_unit_expr(&mut self) -> UnitExpr {
        let mut node = self.parse_unit();

        while let Some(token) = self.peek() {
            let op;
            match token.kind {
                TokenKind::Dot => {
                    self.advance();
                    op = UnitOp::Multiply;
                }
                TokenKind::Slash => {
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
        match self.advance() {
            Some(Token {
                kind: TokenKind::Identifier(id),
                ..
            }) => UnitExpr::Symbol(id.clone()),
            Some(Token { line, column, .. }) => {
                panic!("Line {}:{}: Expected unit identifier", line, column)
            }
            _ => panic!("Expected unit identifier"),
        }
    }

    fn parse_unit_exponent(&mut self) -> Option<f64> {
        if let Some(Token {
            kind: TokenKind::Caret,
            ..
        }) = self.peek()
        {
            self.advance();

            match self.advance() {
                Some(Token {
                    kind: TokenKind::Number(n),
                    ..
                }) => Some(n.clone()),
                Some(Token { line, column, .. }) => {
                    panic!("Line {}:{}: Expected number after ^", line, column)
                }
                _ => panic!("Expected number after ^"),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{UnitExpr, UnitOp};
    use crate::token::{Token, TokenKind};

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_single_unit() {
        let tokens = vec![Token::new(TokenKind::Identifier("m".into()), 1, 1)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr();
        assert_eq!(expr, UnitExpr::Symbol("m".into()));
    }

    #[test]
    fn parse_unit_with_exponent() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("s".into()), 1, 1),
            Token::new(TokenKind::Caret, 1, 2),
            Token::new(TokenKind::Number(2.0), 1, 3),
        ];
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
            Token::new(TokenKind::Identifier("kg".into()), 1, 1),
            Token::new(TokenKind::Dot, 1, 3),
            Token::new(TokenKind::Identifier("m".into()), 1, 4),
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
            Token::new(TokenKind::Identifier("m".into()), 1, 1),
            Token::new(TokenKind::Slash, 1, 2),
            Token::new(TokenKind::Identifier("s".into()), 1, 3),
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
            Token::new(TokenKind::Identifier("kg".into()), 1, 1),
            Token::new(TokenKind::Dot, 1, 3),
            Token::new(TokenKind::Identifier("m".into()), 1, 4),
            Token::new(TokenKind::Slash, 1, 5),
            Token::new(TokenKind::Identifier("s".into()), 1, 6),
            Token::new(TokenKind::Caret, 1, 7),
            Token::new(TokenKind::Number(2.0), 1, 8),
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
        let tokens = vec![
            Token::new(TokenKind::Identifier("s".into()), 1, 1),
            Token::new(TokenKind::Caret, 1, 2),
        ];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }

    #[test]
    #[should_panic(expected = "Expected number after ^")]
    fn parse_unit_after_exponent_should_panic() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("s".into()), 1, 1),
            Token::new(TokenKind::Caret, 1, 2),
            Token::new(TokenKind::Identifier("m".into()), 1, 3),
        ];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }

    #[test]
    #[should_panic(expected = "Expected number after ^")]
    fn parse_symbol_after_exponent_should_panic() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("s".into()), 1, 1),
            Token::new(TokenKind::Caret, 1, 2),
            Token::new(TokenKind::Caret, 1, 3),
        ];
        let mut parser = make_parser(tokens);
        parser.parse_unit_expr();
    }
}
