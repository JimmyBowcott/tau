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
        let tokens = vec![Token::Number(42.0)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr();
        assert_eq!(expr, Expr::Number(42.0));
    }

    #[test]
    fn parse_identifier() {
        let tokens = vec![Token::Identifier("x".into())];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr();
        assert_eq!(expr, Expr::Identifier("x".into()));
    }

    #[test]
    fn parse_simple_addition() {
        let tokens = vec![Token::Number(2.0), Token::Plus, Token::Number(3.0)];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr();

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
            Token::Number(2.0),
            Token::Plus,
            Token::Number(3.0),
            Token::Star,
            Token::Number(4.0),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr();

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
            Token::LParen,
            Token::Number(2.0),
            Token::Plus,
            Token::Number(3.0),
            Token::RParen,
            Token::Star,
            Token::Number(4.0),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr();

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
            Token::Number(2.0),
            Token::Caret,
            Token::Number(3.0),
            Token::Caret,
            Token::Number(2.0),
        ];
        let mut parser = make_parser(tokens);
        let expr = parser.parse_expr();

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
    #[should_panic(expected = "Expected number or variable")]
    fn parse_empty_should_panic() {
        let tokens = vec![];
        let mut parser = make_parser(tokens);
        parser.parse_expr();
    }
}
