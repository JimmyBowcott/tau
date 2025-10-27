use crate::{analysis::Analyser, runtime::Env};

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
pub enum Expr {
    Number(f64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

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

    pub fn validate(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            Expr::Number(_) => Ok(()),
            Expr::Identifier(name) => ctx.symbols.check_declared(name),
            Expr::Binary { left, right, .. } => {
                left.validate(ctx)?;
                right.validate(ctx)
            }
        }
    }
}

impl UnitExpr {
    pub fn validate(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            UnitExpr::Symbol(s) => ctx.units.validate(s),
            UnitExpr::Power { base, .. } => base.validate(ctx),
            UnitExpr::Binary { left, right, .. } => {
                left.validate(ctx)?;
                right.validate(ctx)
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
                env.insert(name.clone(), val);
            }
            Stmt::Print(expr) => {
                let val = expr.eval(env);
                println!("{}", val);
            }
        }
    }

    pub fn analyse(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            Stmt::Let { name, value, unit } => {
                ctx.symbols.declare(name)?;
                if let Some(unit) = unit { unit.validate(ctx)?; }
                value.validate(ctx)?;
                ctx.symbols.define(name);
                Ok(())
            }
            Stmt::Print(expr) | Stmt::Expr(expr) => {
                expr.validate(ctx)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOp, Expr, Stmt};
    use crate::runtime::Env;

    fn env() -> Env {
        Env::new()
    }

    #[test]
    fn eval_number() {
        let mut env = env();
        let expr = Expr::Number(42.0);
        assert_eq!(expr.eval(&mut env), 42.0);
    }

    #[test]
    fn eval_binary_add() {
        let mut env = env();
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(expr.eval(&mut env), 5.0);
    }

    #[test]
    fn eval_binary_power() {
        let mut env = env();
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Power,
            right: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(expr.eval(&mut env), 8.0);
    }

    #[test]
    fn eval_identifier() {
        let mut env = env();
        env.vars.insert("x".to_string(), 7.0);
        let expr = Expr::Identifier("x".into());
        assert_eq!(expr.eval(&mut env), 7.0);
    }

    #[test]
    fn exec_let_statement() {
        let mut env = env();
        let stmt = Stmt::Let {
            name: "x".into(),
            value: Expr::Number(10.0),
            unit: None,
        };
        stmt.exec(&mut env);
        assert_eq!(*env.vars.get("x").unwrap(), 10.0);
    }
}
