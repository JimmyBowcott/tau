use std::fmt;
use crate::{error::Error, runtime::Env};

#[derive(Debug, Clone, Eq)]
pub struct Spanned<T> {
    pub node: T,
    pub line: usize,
    pub column: usize,
}

impl<T> Spanned<T> {
    pub fn new(node: T, line: usize, column: usize) -> Self {
        Self { node, line, column }
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(self: &Self, rhs: &Self) -> bool {
        self.node == rhs.node
    }
}

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

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

pub type Expr = Spanned<ExprKind>;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        unit: Option<UnitExpr>,
        value: Expr,
    },
    Print(Expr),
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "add",
            BinaryOp::Subtract => "subtract",
            BinaryOp::Multiply => "multiply",
            BinaryOp::Divide => "divide",
            BinaryOp::Power => "power",
        };
        // let s = format!("{:?}", self).to_lowercase();
        write!(f, "{}", s)
    }
}

impl Expr {
    fn eval(&self, env: &mut Env) -> Result<f64, Error> {
        match &self.node {
            ExprKind::Number(n) => Ok(*n),
            ExprKind::Binary { left, op, right } => {
                let l = left.eval(env)?;
                let r = right.eval(env)?;
                match op {
                    BinaryOp::Add => Ok(l + r),
                    BinaryOp::Subtract => Ok(l - r),
                    BinaryOp::Multiply => Ok(l * r),
                    BinaryOp::Divide => Ok(l / r),
                    BinaryOp::Power => Ok(l.powf(r)),
                }
            }
            ExprKind::Identifier(name) => {
                if let Some(value) = env.get(&name) {
                    Ok(*value)
                } else {
                    Err(self.error(format!("Unknown variable {}", name)))
                }
            }
        }
    }
}

impl Stmt {
    pub fn exec(&self, env: &mut Env) -> Result<(), Error> {
        match self {
            Stmt::Expr(expr) => {
                expr.eval(env)?;
            }
            Stmt::Let { name, value, .. } => {
                let val = value.eval(env)?;
                env.insert(name.clone(), val);
            }
            Stmt::Print(expr) => {
                let val = expr.eval(env)?;
                println!("{}", val);
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOp, Expr, ExprKind, Stmt};
    use crate::runtime::Env;

    fn env() -> Env {
        Env::new()
    }

    #[test]
    fn eval_number() {
        let mut env = env();
        let expr = Expr::new(ExprKind::Number(42.0), 0, 0);
        assert_eq!(expr.eval(&mut env).unwrap(), 42.0);
    }

    #[test]
    fn eval_binary_add() {
        let mut env = env();
        let expr = Expr::new(ExprKind::Binary {
            left: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::new(ExprKind::Number(3.0), 0, 0)),
        }, 0, 0);
        assert_eq!(expr.eval(&mut env).unwrap(), 5.0);
    }

    #[test]
    fn eval_binary_power() {
        let mut env = env();
        let expr = Expr::new(ExprKind::Binary {
            left: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
            op: BinaryOp::Power,
            right: Box::new(Expr::new(ExprKind::Number(3.0), 0, 0)),
        }, 0, 0);
        assert_eq!(expr.eval(&mut env).unwrap(), 8.0);
    }

    #[test]
    fn eval_identifier() {
        let mut env = env();
        env.insert("x".to_string(), 7.0);
        let expr = Expr::new(ExprKind::Identifier("x".into()), 0, 0);
        assert_eq!(expr.eval(&mut env).unwrap(), 7.0);
    }

    #[test]
    fn exec_let_statement() {
        let mut env = env();
        let stmt = Stmt::Let {
            name: "x".into(),
            value: Expr::new(ExprKind::Number(10.0), 0, 0),
            unit: None,
        };
        stmt.exec(&mut env).unwrap();
        assert_eq!(*env.get("x").unwrap(), 10.0);
    }

    #[test]
    fn binaryop_displays_correctly() {
        let expected = [
            "add",
            "subtract",
            "divide",
            "multiply",
            "power"
        ];
        let actual = [
            BinaryOp::Add.to_string(),
            BinaryOp::Subtract.to_string(),
            BinaryOp::Divide.to_string(),
            BinaryOp::Multiply.to_string(),
            BinaryOp::Power.to_string(),
        ];
        assert_eq!(expected, actual);
    }
}
