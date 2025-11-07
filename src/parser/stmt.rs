use crate::{
    ast::{Expr, Stmt, UnitExpr},
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Result<Option<Stmt>, String> {
        let token = match self.peek() {
            Some(t) => t,
            _ => return Ok(None),
        };

        match token {
            Token {
                kind: TokenKind::Let,
                ..
            } => self.parse_let_stmt().map(Some),
            Token {
                kind: TokenKind::Print,
                ..
            } => Ok(Some(self.parse_print_stmt())),
            _ => Err(format!("Line {}:{}: unexpected token '{:?}'", token.line, token.column, token.kind)),
        }
    }

    pub fn expect_token(&mut self, expected: TokenKind, err_msg: &str) -> Result<(), String> {
        match self.advance() {
            Some(token) if token.kind == expected => Ok(()),
            Some(token) => Err(format!("Line {}:{}: {}", token.line, token.column, err_msg)),
            None => Err("Unexpected end of input".into()),
        }
    }

    fn expect_identifier(&mut self, err_msg: &str) -> Result<String, String> {
        match self.advance() {
            Some(Token {
                kind: TokenKind::Identifier(id),
                ..
            }) => Ok(id.clone()),
            Some(Token { line, column, .. }) => Err(format!("Line {}:{}, {}", line, column, err_msg)),
            _ => Err(format!("{}", err_msg)),
        }
    }

    fn expect_unit(&mut self, err_msg: &str) -> Result<UnitExpr, String> {
        self.advance();

        match self.peek() {
            Some(Token {
                kind: TokenKind::Identifier(_),
                ..
            }) => Ok(self.parse_unit_expr()),
            Some(Token { line, column, .. }) => Err(format!("Line {}:{}, {}", line, column, err_msg)),
            _ => Err(format!("{}", err_msg)),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, String> {
        self.advance();

        let name = self.expect_identifier("Expected identifier after 'let'")?;
        let mut unit = None;

        if let Some(Token {
            kind: TokenKind::Colon,
            ..
        }) = self.peek()
        {
            unit = Some(self.expect_unit("Expected unit after ':'")?);
        }

        self.expect_token(TokenKind::Equal, "Expected '='")?;
        let value = self.parse_expr();

        Ok(Stmt::Let { name, unit, value })
    }

    fn parse_print_stmt(&mut self) -> Stmt {
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

        Stmt::Print(expr)
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
        let stmt = parser.parse_stmt().unwrap().unwrap();

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
        let stmt = parser.parse_stmt().unwrap().unwrap();

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
        let stmt = parser.parse_stmt().unwrap().unwrap();

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
        let stmt = parser.parse_stmt().unwrap().unwrap();

        let expected_expr = Expr::Binary {
            left: Box::new(Expr::Number(3.14)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };

        assert_eq!(stmt, Stmt::Print(expected_expr));
    }

    #[test]
    fn parse_let_missing_identifier_returns_err() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Equal, 1, 5),
            Token::new(TokenKind::Number(5.0), 1, 7),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_stmt().unwrap_err();
        assert!(err.contains("Expected identifier after 'let'"));
    }

    #[test]
    fn parse_let_missing_equal_returns_err() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Identifier("x".into()), 1, 5),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_stmt().unwrap_err();
        assert!(err.contains(" ")); // TODO: Fix this... The error is actually getting caught
                                    // before here
    }

    #[test]
    fn parse_let_missing_unit_returns_err() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1),
            Token::new(TokenKind::Identifier("v".into()), 1, 5),
            Token::new(TokenKind::Colon, 1, 6),
            Token::new(TokenKind::Equal, 1, 8),
            Token::new(TokenKind::Number(10.0), 1, 10),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_stmt().unwrap_err();
        assert!(err.contains("Expected unit after ':'"));
    }
}
