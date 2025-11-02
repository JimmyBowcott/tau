use core::fmt;

use crate::{
    ast::{Expr, Stmt, UnitExpr},
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek()? {
            Token {
                kind: TokenKind::Let,
                ..
            } => self.parse_let_stmt(),
            Token {
                kind: TokenKind::Print,
                ..
            } => self.parse_print_stmt(),
            _ => None,
        }
    }

    pub fn expect_token(&mut self, expected: TokenKind, err_msg: &str) {
        if let Some(token) = self.advance() {
            if token.kind != expected {
                panic!("Line {}:{}: {}", token.line, token.column, err_msg);
            }
        }
    }

    fn expect_identifier(&mut self, err_msg: &str) -> String {
        match self.advance() {
            Some(Token {
                kind: TokenKind::Identifier(id),
                ..
            }) => id.clone(),
            Some(Token { line, column, .. }) => panic!("Line {}:{}, {}", line, column, err_msg),
            _ => panic!("{}", err_msg),
        }
    }

    fn expect_unit(&mut self, err_msg: &str) -> UnitExpr {
        self.advance();

        match self.peek() {
            Some(Token {
                kind: TokenKind::Identifier(_),
                ..
            }) => self.parse_unit_expr(),
            Some(Token { line, column, .. }) => panic!("Line {}:{}, {}", line, column, err_msg),
            _ => panic!("{}", err_msg),
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.advance();

        let name = self.expect_identifier("Expected identifier after 'let'");

        let mut unit = None;

        if let Some(Token {
            kind: TokenKind::Colon,
            ..
        }) = self.peek()
        {
            unit = Some(self.expect_unit("Expected unit after ':'"));
        }

        self.expect_token(TokenKind::Equal, "Expected '='");
        let value = self.parse_expr();

        Some(Stmt::Let { name, unit, value })
    }

    fn parse_print_stmt(&mut self) -> Option<Stmt> {
        self.advance();
        let expr: Expr;

        if let Some(Token {
            kind: TokenKind::Identifier(name),
            ..
        }) = self.peek()
        {
            expr = Expr::Identifier(name.clone());
            self.advance();
        } else {
            expr = self.parse_expr();
        }

        Some(Stmt::Print(expr))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOp, Expr, Stmt, UnitExpr, UnitOp};
    use crate::parser::Parser;
    use crate::token::{Token, TokenKind};

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_simple_let() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Identifier("x".into()), 1, 5),
            Token::new(TokenKind::Equal, 1, 7),
            Token::new(TokenKind::Number(42.0), 1, 9),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap();

        assert_eq!(
            stmt,
            Stmt::Let {
                name: "x".into(),
                unit: None,
                value: Expr::Number(42.0),
            }
        );
    }

    #[test]
    fn parse_let_with_unit() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Identifier("v".into()), 1, 5),
            Token::new(TokenKind::Colon, 1, 6),
            Token::new(TokenKind::Identifier("m".into()), 1, 7),
            Token::new(TokenKind::Slash, 1, 8),
            Token::new(TokenKind::Identifier("s".into()), 1, 9),
            Token::new(TokenKind::Equal, 1, 11),
            Token::new(TokenKind::Number(10.0), 1, 13),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap();

        let expected_unit = UnitExpr::Binary {
            left: Box::new(UnitExpr::Symbol("m".into())),
            op: UnitOp::Divide,
            right: Box::new(UnitExpr::Symbol("s".into())),
        };

        assert_eq!(
            stmt,
            Stmt::Let {
                name: "v".into(),
                unit: Some(expected_unit),
                value: Expr::Number(10.0),
            }
        );
    }

    #[test]
    fn parse_print_identifier() {
        let tokens = vec![
            Token::new(TokenKind::Print, 1, 1),
            Token::new(TokenKind::Identifier("x".into()), 1, 7),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap();

        assert_eq!(stmt, Stmt::Print(Expr::Identifier("x".into())));
    }

    #[test]
    fn parse_print_expr() {
        let tokens = vec![
            Token::new(TokenKind::Print, 1, 1),
            Token::new(TokenKind::Number(3.14), 1, 7),
            Token::new(TokenKind::Plus, 1, 11),
            Token::new(TokenKind::Number(2.0), 1, 13),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap();

        let expected_expr = Expr::Binary {
            left: Box::new(Expr::Number(3.14)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };

        assert_eq!(stmt, Stmt::Print(expected_expr));
    }

    #[test]
    #[should_panic(expected = "Expected identifier after 'let'")]
    fn parse_let_missing_identifier_should_panic() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Equal, 1, 5),
            Token::new(TokenKind::Number(5.0), 1, 7),
        ];
        let mut parser = make_parser(tokens);
        parser.parse_stmt();
    }

    #[test]
    #[should_panic(expected = "Expected expression after '='")]
    fn parse_let_missing_equal_should_panic() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Identifier("x".into()), 1, 5),
        ];
        let mut parser = make_parser(tokens);
        parser.parse_stmt();
    }

    #[test]
    #[should_panic(expected = "Expected unit after ':'")]
    fn parse_let_missing_unit_should_panic() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Identifier("v".into()), 1, 5),
            Token::new(TokenKind::Colon, 1, 6),
            Token::new(TokenKind::Equal, 1, 8),
            Token::new(TokenKind::Number(10.0), 1, 10),
        ];
        let mut parser = make_parser(tokens);
        parser.parse_stmt();
    }
}
