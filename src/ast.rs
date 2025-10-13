use crate::runtime::Env;

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
    Expr(Expr),
    Let {
        name: String,
        unit: Option<UnitExpr>,
        value: Expr,
    },
    Print(Expr),
}

impl Expr {
    fn eval(&self, env: &mut Env) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Binary { left, op, right } => {
                let l = left.eval(env);
                let r = right.eval(env);
                match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Subtract => l - r,
                    BinaryOp::Multiply => l * r,
                    BinaryOp::Divide => l / r,
                    BinaryOp::Power => l.powf(r),
                }
            }
            Expr::Identifier(name) => {
                if let Some(value) = env.vars.get(name) {
                    *value
                } else {
                    panic!("Unknown variable {}", name);
                }
            }
        }
    }
}

impl Stmt {
    pub fn exec(&self, env: &mut Env) {
        match self {
            Stmt::Expr(expr) => {
                expr.eval(env);
            }
            Stmt::Let { name, value, .. } => {
                let val = value.eval(env);
                env.vars.insert(name.clone(), val);
            }
            Stmt::Print(expr) => {
                let val = expr.eval(env);
                println!("{}", val);
            }
        }
    }
}
