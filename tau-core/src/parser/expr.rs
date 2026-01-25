use crate::{
    ast::{BinaryOp, Expr, ExprKind},
    error::Error,
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_expr(&mut self) -> Result<Expr, Error> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr, Error> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.peek() {
                Some(Token {
                    kind: TokenKind::Plus,
                    ..
                }) => (1, 2, BinaryOp::Add),
                Some(Token {
                    kind: TokenKind::Minus,
                    ..
                }) => (1, 2, BinaryOp::Subtract),
                Some(Token {
                    kind: TokenKind::Star,
                    ..
                }) => (3, 4, BinaryOp::Multiply),
                Some(Token {
                    kind: TokenKind::Slash,
                    ..
                }) => (3, 4, BinaryOp::Divide),
                Some(Token {
                    kind: TokenKind::Caret,
                    ..
                }) => (5, 4, BinaryOp::Power),
                _ => break,
            };

            let (lbp, rbp, bop) = op;
            if lbp < min_bp {
                break;
            }

            if let Some(tok) = self.advance().cloned() {
                let rhs = self.parse_expr_bp(rbp)?;
                lhs = Expr::new(
                    ExprKind::Binary {
                        left: Box::new(lhs),
                        op: bop,
                        right: Box::new(rhs),
                    },
                    tok.line,
                    tok.column,
                );
            }
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Expr, Error> {
        // TODO: Fix this return
        let tok = self.peek().cloned().ok_or(Error::new(
            self.pos,
            1,
            "Unexpected end of input".into(),
        ))?;
        self.advance();

        match &tok.kind {
            TokenKind::Identifier(id) => Ok(Expr::new(
                ExprKind::Identifier(id.clone()),
                tok.line,
                tok.column,
            )),

            TokenKind::Number(n) => Ok(Expr::new(ExprKind::Number(*n), tok.line, tok.column)),

            TokenKind::LParen => {
                let expr = self.parse_expr()?;
                match self.advance() {
                    Some(Token {
                        kind: TokenKind::RParen,
                        ..
                    }) => Ok(expr),
                    Some(t) => Err(Error::new(t.line, t.column, "Expected ')'".into())),
                    None => Err(Error::new(tok.line, tok.column, "Expected ')'".into())),
                }
            }

            _ => Err(Error::new(
                tok.line,
                tok.column,
                "Expected number or identifier".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOp, ExprKind};
    use crate::token::Token;

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_number() {
        let tokens = vec![Token::new(TokenKind::Number(42.0), 1, 1, 2)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr.node, ExprKind::Number(42.0));
    }

    #[test]
    fn parse_identifier() {
        let tokens = vec![Token::new(TokenKind::Identifier("x".into()), 1, 1, 1)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr.node, ExprKind::Identifier("x".into()));
    }

    #[test]
    fn parse_simple_addition() {
        let tokens = vec![
            Token::new(TokenKind::Number(2.0), 1, 1, 1),
            Token::new(TokenKind::Plus, 1, 2, 1),
            Token::new(TokenKind::Number(3.0), 1, 3, 1),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr.node,
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::new(ExprKind::Number(3.0), 0, 0)),
            }
        );
    }

    #[test]
    fn parse_precedence() {
        // 2 + 3 * 4 => 2 + (3*4)
        let tokens = vec![
            Token::new(TokenKind::Number(2.0), 1, 1, 1),
            Token::new(TokenKind::Plus, 1, 2, 1),
            Token::new(TokenKind::Number(3.0), 1, 3, 1),
            Token::new(TokenKind::Star, 1, 4, 1),
            Token::new(TokenKind::Number(4.0), 1, 5, 1),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr.node,
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::new(
                    ExprKind::Binary {
                        left: Box::new(Expr::new(ExprKind::Number(3.0), 0, 0)),
                        op: BinaryOp::Multiply,
                        right: Box::new(Expr::new(ExprKind::Number(4.0), 0, 0)),
                    },
                    0,
                    0
                )),
            }
        );
    }

    #[test]
    fn parse_parentheses() {
        // (2 + 3) * 4
        let tokens = vec![
            Token::new(TokenKind::LParen, 1, 1, 1),
            Token::new(TokenKind::Number(2.0), 1, 2, 1),
            Token::new(TokenKind::Plus, 1, 3, 1),
            Token::new(TokenKind::Number(3.0), 1, 4, 1),
            Token::new(TokenKind::RParen, 1, 5, 1),
            Token::new(TokenKind::Star, 1, 6, 1),
            Token::new(TokenKind::Number(4.0), 1, 7, 1),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr.node,
            ExprKind::Binary {
                left: Box::new(Expr::new(
                    ExprKind::Binary {
                        left: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::new(ExprKind::Number(3.0), 0, 0)),
                    },
                    0,
                    0
                )),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::new(ExprKind::Number(4.0), 0, 0)),
            }
        );
    }

    #[test]
    fn parse_power_associativity() {
        // 2 ^ 3 ^ 2 = 2 ^ (3 ^ 2)
        let tokens = vec![
            Token::new(TokenKind::Number(2.0), 1, 1, 1),
            Token::new(TokenKind::Caret, 1, 2, 1),
            Token::new(TokenKind::Number(3.0), 1, 3, 1),
            Token::new(TokenKind::Caret, 1, 4, 1),
            Token::new(TokenKind::Number(2.0), 1, 5, 1),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr.node,
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
                op: BinaryOp::Power,
                right: Box::new(Expr::new(
                    ExprKind::Binary {
                        left: Box::new(Expr::new(ExprKind::Number(3.0), 0, 0)),
                        op: BinaryOp::Power,
                        right: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
                    },
                    0,
                    0
                )),
            }
        );
    }

    #[test]
    fn parse_empty_should_err() {
        let tokens = vec![];
        let mut parser = make_parser(tokens);
        assert!(parser.parse_expr().is_err());
    }
}
