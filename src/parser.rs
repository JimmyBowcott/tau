use crate::lexer::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitOp {
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum UnitExpr {
    Symbol(String),
    Power {
        base: Box<UnitExpr>,
        exponent: f64,
    },
    Binary {
        left: Box<UnitExpr>,
        op: UnitOp,
        right: Box<UnitExpr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VariableDecl {
        name: String,
        unit: Option<UnitExpr>,
        value: Expr,
    },
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.current >= self.tokens.len() {
            return None;
        }
        let tok = &self.tokens[self.current];
        self.current += 1;
        Some(tok)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn parse_operator(&mut self) -> Option<BinaryOp> {
        if let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.advance();
                    return Some(BinaryOp::Add);
                }
                Token::Minus => {
                    self.advance();
                    return Some(BinaryOp::Subtract);
                }
                Token::Star => {
                    self.advance();
                    return Some(BinaryOp::Multiply);
                }
                Token::Slash => {
                    self.advance();
                    return Some(BinaryOp::Divide);
                }
                _ => return None,
            }
        } else {
            panic!("Expected ;")
        }
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

    fn parse_expr(&mut self) -> Expr {
        let mut res = self.parse_primary();

        while let Some(op) = self.parse_operator() {
            res = Expr::Binary {
                left: Box::new(res),
                op,
                right: Box::new(self.parse_primary()),
            }
        }
        res
    }

    fn parse_unit_base(&mut self) -> UnitExpr {
        let base = if let Some(Token::Identifier(id)) = self.advance() {
            UnitExpr::Symbol(id.clone())
        } else {
            panic!("Expected unit identifier");
        };
        base
    }

    fn parse_unit_exponent(&mut self) -> Option<f64> {
        if let Some(Token::Caret) = self.peek() {
            self.advance();
            if let Some(Token::Number(n)) = self.advance() {
                return Some(n.clone());
            } else {
                panic!("Expected number after ^");
            }
        } else {
            return None;
        }
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

    fn parse_unit_expr(&mut self) -> UnitExpr {
        let mut node = self.parse_unit();

        while let Some(token) = self.peek() {
            let op;
            match token {
                Token::Dot => {
                    self.advance();
                    op = UnitOp::Multiply;
                }
                Token::Slash => {
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

    fn parse_statement(&mut self) -> Option<Stmt> {
        match self.peek()? {
            // TODO: Move this somewhere...
            Token::Let => {
                self.advance();

                let name = if let Some(Token::Identifier(id)) = self.advance() {
                    id.clone()
                } else {
                    panic!("Expected identifier after 'let'");
                };

                let mut unit = None;

                if let Some(Token::Colon) = self.peek() {
                    self.advance();

                    if let Some(Token::Identifier(_)) = self.peek() {
                        unit = Some(self.parse_unit_expr());
                    } else {
                        panic!("Expected unit after ':'")
                    };
                }

                if let Some(Token::Equal) = self.advance() {
                    let value = self.parse_expr();
                    Some(Stmt::VariableDecl { name, unit, value })
                } else {
                    panic!("Expected expression after '='")
                }
            }
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Option<Stmt> {
        self.parse_statement()
    }
}
