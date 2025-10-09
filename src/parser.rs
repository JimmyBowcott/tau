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

                    if let Some(Token::Identifier(u)) = self.peek() {
                        unit = Some(UnitExpr::Symbol(u.clone()))
                        // TODO: Parse units with *, /, etc. recursively
                        } else {
                        panic!("Expected unit after ':'")
                    };
                }

                if unit.is_some() {
                    self.advance();
                }

                if let Some(Token::Equal) = self.advance() {
                    // TODO: Parse the expression
                    let value = Expr::Number(10.0);
                    Some(Stmt::VariableDecl { name, unit, value })
                } else {
                    panic!("Expected expression after '='")
                }
            }
            _ => None,
        }
    }
}
