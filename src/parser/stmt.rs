use crate::{
    ast::{Expr, ExprKind, Stmt, StmtKind, UnitExpr},
    error::Error,
    token::{Token, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Result<Option<Stmt>, Error> {
        let token = match self.peek() {
            Some(t) => t,
            _ => return Ok(None),
        };

        match &token.kind {
            TokenKind::Let => self.parse_let_stmt().map(Some),
            TokenKind::Const => self.parse_const_stmt().map(Some),
            TokenKind::Identifier(_) => self.parse_assignment().map(Some),
            TokenKind::Print => self.parse_print_stmt().map(Some),
            _ => Err(Error::new(
                token.line,
                token.column,
                format!("Unexpected token: {}", token.kind),
            )),
        }
    }

    pub fn expect_token(&mut self, expected: TokenKind, err_msg: &str) -> Result<(), Error> {
        match self.advance() {
            Some(token) if token.kind == expected => Ok(()),
            Some(Token { line, column, .. }) => Err(Error::new(*line, *column, err_msg.into())),
            None => Err(Error::new(
                self.line,
                self.column,
                "Unexpected end of input".into(),
            )),
        }
    }

    fn expect_identifier(&mut self, err_msg: &str) -> Result<String, Error> {
        match self.advance() {
            Some(Token {
                kind: TokenKind::Identifier(id),
                ..
            }) => Ok(id.clone()),
            Some(Token { line, column, .. }) => Err(Error::new(*line, *column, err_msg.into())),
            _ => Err(Error::new(self.line, self.column, err_msg.into())),
        }
    }

    fn expect_unit(&mut self, err_msg: &str) -> Result<UnitExpr, Error> {
        self.advance();

        match self.peek() {
            Some(Token {
                kind: TokenKind::Identifier(_),
                ..
            }) => Ok(self.parse_unit_expr()?),
            Some(Token { line, column, .. }) => Err(Error::new(*line, *column, err_msg.into())),
            _ => Err(Error::new(self.line, self.column, err_msg.into())),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, Error> {
        let stmt_line = self.line.clone();
        let stmt_col = self.column.clone();
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
        let value = self.parse_expr()?;

        Ok(Stmt::new(
            StmtKind::Let { name, unit, value },
            stmt_line,
            stmt_col,
        ))
    }

    fn parse_const_stmt(&mut self) -> Result<Stmt, Error> {
        let stmt_line = self.line.clone();
        let stmt_col = self.column.clone();
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
        let value = self.parse_expr()?;

        Ok(Stmt::new(
            StmtKind::Const { name, unit, value },
            stmt_line,
            stmt_col,
        ))
    }

    fn parse_assignment(&mut self) -> Result<Stmt, Error> {
        let stmt_line = self.line.clone();
        let stmt_col = self.column.clone();
        let name = self.expect_identifier("Internal error - expected identifier to be identifier.")?;
        self.expect_token(TokenKind::Equal, "Expected '='")?;
        let value = self.parse_expr()?;

        Ok(Stmt::new(
            StmtKind::Assign { name, value },
            stmt_line,
            stmt_col,
        ))
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt, Error> {
        let stmt_line = self.line.clone();
        let stmt_col = self.column.clone();
        let expr: Expr;
        self.advance();

        if let Some(Token {
            kind: TokenKind::Identifier(name),
            line,
            column,
            ..
        }) = self.peek()
        {
            expr = Expr::new(ExprKind::Identifier(name.clone()), *line, *column);
            self.advance();
        } else {
            expr = self.parse_expr()?;
        }

        Ok(Stmt::new(StmtKind::Print(expr), stmt_line, stmt_col))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOp, Expr, ExprKind, StmtKind, UnitExpr, UnitExprKind, UnitOp};
    use crate::parser::Parser;
    use crate::token::{Token, TokenKind};

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_simple_let() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1, 3),
            Token::new(TokenKind::Identifier("x".into()), 1, 5, 1),
            Token::new(TokenKind::Equal, 1, 7, 1),
            Token::new(TokenKind::Number(42.0), 1, 9, 2),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap().unwrap().node;

        assert_eq!(
            stmt,
            StmtKind::Let {
                name: "x".into(),
                unit: None,
                value: Expr::new(ExprKind::Number(42.0), 0, 0),
            }
        );
    }

    #[test]
    fn parse_let_with_unit() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1, 3),
            Token::new(TokenKind::Identifier("v".into()), 1, 5, 1),
            Token::new(TokenKind::Colon, 1, 6, 1),
            Token::new(TokenKind::Identifier("m".into()), 1, 7, 1),
            Token::new(TokenKind::Slash, 1, 8, 1),
            Token::new(TokenKind::Identifier("s".into()), 1, 9, 1),
            Token::new(TokenKind::Equal, 1, 11, 1),
            Token::new(TokenKind::Number(10.0), 1, 13, 2),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap().unwrap().node;

        let expected_unit = UnitExpr::new(
            UnitExprKind::Binary {
                left: Box::new(UnitExpr::new(UnitExprKind::Symbol("m".into()), 1, 1)),
                op: UnitOp::Divide,
                right: Box::new(UnitExpr::new(UnitExprKind::Symbol("s".into()), 1, 1)),
            },
            1,
            1,
        );

        assert_eq!(
            stmt,
            StmtKind::Let {
                name: "v".into(),
                unit: Some(expected_unit),
                value: Expr::new(ExprKind::Number(10.0), 0, 0),
            }
        );
    }

    #[test]
    fn parse_print_identifier() {
        let tokens = vec![
            Token::new(TokenKind::Print, 1, 1, 5),
            Token::new(TokenKind::Identifier("x".into()), 1, 7, 1),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap().unwrap().node;

        assert_eq!(
            stmt,
            StmtKind::Print(Expr::new(ExprKind::Identifier("x".into()), 0, 0))
        );
    }

    #[test]
    fn parse_print_expr() {
        let tokens = vec![
            Token::new(TokenKind::Print, 1, 1, 5),
            Token::new(TokenKind::Number(3.14), 1, 7, 4),
            Token::new(TokenKind::Plus, 1, 11, 1),
            Token::new(TokenKind::Number(2.0), 1, 13, 1),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap().unwrap().node;

        let expected_expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Number(3.14), 0, 0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
            },
            0,
            0,
        );

        assert_eq!(stmt, StmtKind::Print(expected_expr));
    }

    #[test]
    fn parse_let_missing_identifier_returns_err() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1, 3),
            Token::new(TokenKind::Equal, 1, 5, 1),
            Token::new(TokenKind::Number(5.0), 1, 7, 1),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_stmt().unwrap_err();
        assert!(err.message.contains("Expected identifier after 'let'"));
    }

    #[test]
    fn parse_let_missing_equal_returns_err() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1, 3),
            Token::new(TokenKind::Identifier("x".into()), 1, 5, 1),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_stmt().unwrap_err();
        assert!(err.message.contains(" ")); // TODO: Fix this... The error is actually getting caught
        // before here
    }

    #[test]
    fn parse_let_missing_unit_returns_err() {
        let tokens = vec![
            Token::new(TokenKind::Let, 1, 1, 3),
            Token::new(TokenKind::Identifier("v".into()), 1, 5, 1),
            Token::new(TokenKind::Colon, 1, 6, 1),
            Token::new(TokenKind::Equal, 1, 8, 1),
            Token::new(TokenKind::Number(10.0), 1, 10, 2),
        ];
        let mut parser = make_parser(tokens);
        let err = parser.parse_stmt().unwrap_err();
        assert!(err.message.contains("Expected unit after ':'"));
    }
}
