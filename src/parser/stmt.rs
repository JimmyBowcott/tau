use crate::{
    ast::{Expr, Stmt, UnitExpr},
    token::Token,
};

use super::Parser;

impl Parser {
    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek()? {
            Token::Let => self.parse_let_stmt(),
            Token::Print => self.parse_print_stmt(),
            _ => None,
        }
    }

    pub fn expect_token(&mut self, expected: Token, err_msg: &str) {
        if self.advance() != Some(&expected) {
            panic!("{}", err_msg);
        }
    }

    fn expect_identifier(&mut self, err_msg: &str) -> String {
        if let Some(Token::Identifier(id)) = self.advance() {
            id.clone()
        } else {
            panic!("{}", err_msg);
        }
    }

    fn expect_unit(&mut self, err_msg: &str) -> UnitExpr {
        self.advance();

        if let Some(Token::Identifier(_)) = self.peek() {
           self.parse_unit_expr()
        } else {
            panic!("{}", err_msg);
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        self.advance();

        let name = self.expect_identifier("Expected identifier after 'let'");

        let mut unit = None;

        if let Some(Token::Colon) = self.peek() {
            unit = Some(self.expect_unit("Expected unit after ':'"));
        }

        self.expect_token(Token::Equal, "Expected '='");
        let value = self.parse_expr();

        Some(Stmt::Let { name, unit, value })
    }

    fn parse_print_stmt(&mut self) -> Option<Stmt> {
        self.advance();
        let expr: Expr;

        if let Some(Token::Identifier(name)) = self.peek() {
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
    use crate::token::Token;
    use crate::ast::{Expr, Stmt};
    use crate::parser::Parser;
    use crate::ast::{UnitExpr, UnitOp};

    fn make_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn parse_simple_let() {
        let tokens = vec![
            Token::Let,
            Token::Identifier("x".into()),
            Token::Equal,
            Token::Number(42.0),
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
            Token::Let,
            Token::Identifier("v".into()),
            Token::Colon,
            Token::Identifier("m".into()),
            Token::Slash,
            Token::Identifier("s".into()),
            Token::Equal,
            Token::Number(10.0),
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
            Token::Print,
            Token::Identifier("x".into()),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap();

        assert_eq!(
            stmt,
            Stmt::Print(Expr::Identifier("x".into()))
        );
    }

    #[test]
    fn parse_print_expr() {
        let tokens = vec![
            Token::Print,
            Token::Number(3.14),
            Token::Plus,
            Token::Number(2.0),
        ];

        let mut parser = make_parser(tokens);
        let stmt = parser.parse_stmt().unwrap();

        let expected_expr = Expr::Binary {
            left: Box::new(Expr::Number(3.14)),
            op: crate::ast::BinaryOp::Add,
            right: Box::new(Expr::Number(2.0)),
        };

        assert_eq!(
            stmt,
            Stmt::Print(expected_expr)
        );
    }

    #[test]
    #[should_panic(expected = "Expected identifier after 'let'")]
    fn parse_let_missing_identifier_should_panic() {
        let tokens = vec![Token::Let, Token::Equal, Token::Number(5.0)];
        let mut parser = make_parser(tokens);
        parser.parse_stmt();
    }

    #[test]
    #[should_panic(expected = "Expected '='")]
    fn parse_let_missing_equal_should_panic() {
        let tokens = vec![Token::Let, Token::Identifier("x".into())];
        let mut parser = make_parser(tokens);
        parser.parse_stmt();
    }

    #[test]
    #[should_panic(expected = "Expected unit after ':'")]
    fn parse_let_missing_unit_should_panic() {
        let tokens = vec![Token::Let, Token::Identifier("v".into()), Token::Colon, Token::Equal, Token::Number(10.0)];
        let mut parser = make_parser(tokens);
        parser.parse_stmt();
    }
}
