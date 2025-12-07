use crate::{
    ast::{UnitExpr, UnitExprKind, UnitOp},
    error::Error,
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_unit_expr(&mut self) -> Result<UnitExpr, Error> {
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
            node = UnitExpr::new(
                UnitExprKind::Binary {
                    left: Box::new(node.clone()),
                    op,
                    right: Box::new(right),
                },
                node.line,
                node.column,
            );
        }
        Ok(node)
    }

    fn parse_unit(&mut self) -> Result<UnitExpr, Error> {
        let base = self.parse_unit_base()?;
        if let Some(exponent) = self.parse_unit_exponent()? {
            return Ok(UnitExpr::new(
                UnitExprKind::Power {
                    base: Box::new(base.clone()),
                    exponent,
                },
                base.line,
                base.column,
            ));
        }
        Ok(base)
    }

    fn parse_unit_base(&mut self) -> Result<UnitExpr, Error> {
        match self.advance() {
            Some(Token {
                kind: TokenKind::Identifier(id),
                line,
                column,
                ..
            }) => Ok(UnitExpr::new(
                UnitExprKind::Symbol(id.clone()),
                *line,
                *column,
            )),
            Some(Token { line, column, .. }) => Err(Error::new(
                *line,
                *column,
                "Expected unit identifier".into(),
            )),
            // TODO: Add correct location
            _ => Err(Error::new(1, 1, "Expected unit identifier".into())),
        }
    }

    fn parse_unit_exponent(&mut self) -> Result<Option<f64>, Error> {
        if let Some(Token {
            kind: TokenKind::Caret,
            line,
            column,
            ..
        }) = self.peek()
        {
            let line_hack = line.clone();
            let col_hack = column.clone();
            self.advance();

            match self.advance() {
                Some(Token {
                    kind: TokenKind::Number(n),
                    ..
                }) => Ok(Some(n.clone())),
                Some(Token { line, column, .. }) => {
                    Err(Error::new(*line, *column, "Expected number after ^".into()))
                }
                _ => Err(Error::new(
                    line_hack,
                    col_hack,
                    "Expected number after ^".into(),
                )),
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
        assert_eq!(expr, UnitExpr::new(UnitExprKind::Symbol("m".into()), 1, 1));
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
            UnitExpr::new(
                UnitExprKind::Power {
                    base: Box::new(UnitExpr::new(UnitExprKind::Symbol("s".into()), 1, 1)),
                    exponent: 2.0,
                },
                1,
                1
            )
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

        let expected = UnitExpr::new(UnitExprKind::Binary {
            left: Box::new(UnitExpr::new(UnitExprKind::Symbol("N".into()), 1, 1)),
            op: UnitOp::Divide,
            right: Box::new(UnitExpr::new( UnitExprKind::Power {
                base: Box::new(UnitExpr::new(UnitExprKind::Symbol("m".into()), 1, 1)),
                exponent: 2.0,
            }, 1, 1)),
        }, 1, 1);

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
        assert!(err.message.contains("Expected number after ^"));
    }
}
