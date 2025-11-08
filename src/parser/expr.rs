use crate::{
    ast::{BinaryOp, Expr},
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Result<Expr, String> {
        let mut lhs = self.parse_primary()?;

        loop {
            let op = match self.peek() {
                Some(Token { kind: TokenKind::Plus, .. }) => (1, 2, BinaryOp::Add),
                Some(Token { kind: TokenKind::Minus, .. }) => (1, 2, BinaryOp::Subtract),
                Some(Token { kind: TokenKind::Star, .. }) => (3, 4, BinaryOp::Multiply),
                Some(Token { kind: TokenKind::Slash, .. }) => (3, 4, BinaryOp::Divide),
                Some(Token { kind: TokenKind::Caret, .. }) => (5, 4, BinaryOp::Power),
                _ => break,
            };

            let (lbp, rbp, bop) = op;
            if lbp < min_bp {
                break;
            }

            self.advance();
            let rhs = self.parse_expr_bp(rbp)?;
            lhs = Expr::Binary {
                left: Box::new(lhs),
                op: bop,
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token { kind: TokenKind::Identifier(id), .. }) => Ok(Expr::Identifier(id.clone())),
            Some(Token { kind: TokenKind::Number(n), .. }) => Ok(Expr::Number(n.clone())),
            Some(Token { kind: TokenKind::LParen, .. }) => {
                let expr = self.parse_expr()?;
                if let Some(Token { kind: TokenKind::RParen, .. }) = self.advance() {
                    Ok(expr)
                } else {
                    Err("Expected )".into())
                }
            }
            _ => Err("Expected number or variable".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use crate::ast::{Expr, BinaryOp};

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_number() {
        let tokens = vec![Token::new(TokenKind::Number(42.0), 1, 1)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn parse_identifier() {
        let tokens = vec![Token::new(TokenKind::Identifier("x".into()), 1, 1)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();
        assert_eq!(expr, Expr::Identifier("x".into()));
    }

    #[test]
    fn parse_simple_addition() {
        let tokens = vec![
            Token::new(TokenKind::Number(2.0), 1, 1),
            Token::new(TokenKind::Plus, 1, 2),
            Token::new(TokenKind::Number(3.0), 1, 3),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn parse_precedence() {
        // 2 + 3 * 4 => 2 + (3*4)
        let tokens = vec![
            Token::new(TokenKind::Number(2.0), 1, 1),
            Token::new(TokenKind::Plus, 1, 2),
            Token::new(TokenKind::Number(3.0), 1, 3),
            Token::new(TokenKind::Star, 1, 4),
            Token::new(TokenKind::Number(4.0), 1, 5),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(3.0)),
                    op: BinaryOp::Multiply,
                    right: Box::new(Expr::Number(4.0)),
                }),
            }
        );
    }

    #[test]
    fn parse_parentheses() {
        // (2 + 3) * 4
        let tokens = vec![
            Token::new(TokenKind::LParen, 1, 1),
            Token::new(TokenKind::Number(2.0), 1, 2),
            Token::new(TokenKind::Plus, 1, 3),
            Token::new(TokenKind::Number(3.0), 1, 4),
            Token::new(TokenKind::RParen, 1, 5),
            Token::new(TokenKind::Star, 1, 6),
            Token::new(TokenKind::Number(4.0), 1, 7),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(2.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(3.0)),
                }),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::Number(4.0)),
            }
        );
    }

    #[test]
    fn parse_power_associativity() {
        // 2 ^ 3 ^ 2 = 2 ^ (3 ^ 2)
        let tokens = vec![
            Token::new(TokenKind::Number(2.0), 1, 1),
            Token::new(TokenKind::Caret, 1, 2),
            Token::new(TokenKind::Number(3.0), 1, 3),
            Token::new(TokenKind::Caret, 1, 4),
            Token::new(TokenKind::Number(2.0), 1, 5),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr().unwrap();

        assert_eq!(
            expr,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Power,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(3.0)),
                    op: BinaryOp::Power,
                    right: Box::new(Expr::Number(2.0)),
                }),
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
