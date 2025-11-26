use crate::{
    ast::{UnitExpr, UnitOp},
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_unit_expr(&mut self) -> Result<UnitExpr, String> {
        let mut node = self.parse_unit()?;

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
            let right = self.parse_unit()?;
            node = UnitExpr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }
        Ok(node)
    }

    fn parse_unit(&mut self) -> Result<UnitExpr, String> {
        let base = self.parse_unit_base()?;
        if let Some(exponent) = self.parse_unit_exponent()? {
            return Ok(UnitExpr::Power {
                base: Box::new(base),
                exponent,
            });
        }
        Ok(base)
    }

    fn parse_unit_base(&mut self) -> Result<UnitExpr, String> {
        match self.advance() {
            Some(Token {
                kind: TokenKind::Identifier(id),
                ..
            }) => Ok(UnitExpr::Symbol(id.clone())),
            Some(Token { line, column, .. }) => {
                Err(format!("Line {}:{}: Expected unit identifier", line, column))
            }
            _ => Err("Expected unit identifier".into()),
        }
    }

    fn parse_unit_exponent(&mut self) -> Result<Option<f64>, String> {
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
                }) => Ok(Some(n.clone())),
                Some(Token { line, column, .. }) => {
                    Err(format!("Line {}:{}: Expected number after ^", line, column))
                }
                _ => Err("Expected number after ^".into()),
            }
        } else {
            Ok(None)
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
        let tokens = vec![Token::new(TokenKind::Identifier("m".into()), 1, 1, 1)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr().unwrap();
        assert_eq!(expr, UnitExpr::Symbol("m".into()));
    }

    #[test]
    fn parse_unit_with_exponent() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("s".into()), 1, 1, 1),
            Token::new(TokenKind::Caret, 1, 2, 1),
            Token::new(TokenKind::Number(2.0), 1, 3, 1),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr().unwrap();
        assert_eq!(
            expr,
            UnitExpr::Power {
                base: Box::new(UnitExpr::Symbol("s".into())),
                exponent: 2.0,
            }
        );
    }

    #[test]
    fn parse_complex_unit() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("N".into()), 1, 1, 1),
            Token::new(TokenKind::Slash, 1, 2, 1),
            Token::new(TokenKind::Identifier("m".into()), 1, 3, 1),
            Token::new(TokenKind::Caret, 1, 4, 1),
            Token::new(TokenKind::Number(2.0), 1, 5, 1),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_unit_expr().unwrap();

        let expected = UnitExpr::Binary {
            left: Box::new(UnitExpr::Symbol("N".into())),
            op: UnitOp::Divide,
            right: Box::new(UnitExpr::Power {
                base: Box::new(UnitExpr::Symbol("m".into())),
                exponent: 2.0,
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parse_empty_unit_should_err() {
        let tokens = vec![];
        let mut parser = make_parser(tokens);
        assert!(parser.parse_unit_expr().is_err());
    }

    #[test]
    fn parse_unit_missing_exponent_should_err() {
        let tokens = vec![
            Token::new(TokenKind::Identifier("s".into()), 1, 1, 1),
            Token::new(TokenKind::Caret, 1, 2, 1),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_unit_expr().unwrap_err();
        assert_eq!(err, "Expected number after ^");
    }
}
