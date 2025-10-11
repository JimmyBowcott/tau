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
