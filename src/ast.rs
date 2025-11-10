use std::fmt;

use crate::{
    analysis::{Analyser, units::Dimension},
    runtime::Env,
};

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

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{:?}", self).to_lowercase();
        write!(f, "{}", s)
    }
}

impl Expr {
    fn eval(&self, env: &mut Env) -> Result<f64, String> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::Binary { left, op, right } => {
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
            Expr::Identifier(name) => {
                if let Some(value) = env.get(name) {
                    Ok(*value)
                } else {
                    Err(format!("Unknown variable {}", name))
                }
            }
        }
    }

    pub fn validate(&self, ctx: &mut Analyser) -> Result<(), String> {
        self.check_declared(ctx)?;
        self.check_dimension(ctx)?;
        Ok(())
    }

    fn check_declared(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            Expr::Number(_) => Ok(()),
            Expr::Identifier(name) => ctx.symbols.check_declared(name),
            Expr::Binary { left, right, .. } => {
                left.validate(ctx)?;
                right.validate(ctx)
            }
        }
    }

    fn check_dimension(&self, ctx: &mut Analyser) -> Result<Dimension, String> {
        match self {
            Expr::Number(_) => Ok(Dimension::new([0; 7])),

            Expr::Identifier(name) => ctx
                .symbols
                .get_dimension(name)
                .ok_or_else(|| format!("Undeclared variable '{}'", name)),

            Expr::Binary { left, op, right } => {
                let ldim = left.check_dimension(ctx)?;
                let rdim = right.check_dimension(ctx)?;

                match op {
                    BinaryOp::Add | BinaryOp::Subtract => {
                        if ldim == rdim {
                            Ok(ldim)
                        } else {
                            Err(format!(
                                "Unit mismatch: cannot {:?} {} and {}",
                                op,
                                ldim,
                                rdim,
                            ))
                        }
                    }

                    BinaryOp::Multiply => Ok(ldim.add(&rdim)),
                    BinaryOp::Divide => Ok(ldim.sub(&rdim)),
                    BinaryOp::Power => {
                        match **right {
                            Expr::Number(exp_val) => {
                                if (exp_val.fract()).abs() > std::f64::EPSILON {
                                    return Err(format!(
                                        "Non-integer exponent {} not allowed for units",
                                        exp_val
                                    ));
                                }
                                let n = exp_val as i32;
                                Ok(ldim.scale(n as f64))
                            }
                            _ => Err("Exponent must be a dimensionless numeric literal".into()),
                        }
                    }
                }
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
    pub fn exec(&self, env: &mut Env) -> Result<(), String> {
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

    pub fn analyse(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            Stmt::Let { name, value, unit } => {
                if let Some(u) = unit {
                    u.validate(ctx)?;
                }

                let dimension = match unit {
                    Some(u) => ctx.get_dimension(u)?,
                    None => Dimension::new([0; 7]),
                };

                ctx.symbols.declare(name, dimension)?;
                value.validate(ctx)?;
                ctx.symbols.define(name);
                Ok(())
            }
            Stmt::Print(expr) | Stmt::Expr(expr) => expr.validate(ctx),
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
        assert_eq!(expr.eval(&mut env).unwrap(), 42.0);
    }

    #[test]
    fn eval_binary_add() {
        let mut env = env();
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Add,
            right: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(expr.eval(&mut env).unwrap(), 5.0);
    }

    #[test]
    fn eval_binary_power() {
        let mut env = env();
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            op: BinaryOp::Power,
            right: Box::new(Expr::Number(3.0)),
        };
        assert_eq!(expr.eval(&mut env).unwrap(), 8.0);
    }

    #[test]
    fn eval_identifier() {
        let mut env = env();
        env.insert("x".to_string(), 7.0);
        let expr = Expr::Identifier("x".into());
        assert_eq!(expr.eval(&mut env).unwrap(), 7.0);
    }

    #[test]
    fn exec_let_statement() {
        let mut env = env();
        let stmt = Stmt::Let {
            name: "x".into(),
            value: Expr::Number(10.0),
            unit: None,
        };
        stmt.exec(&mut env).unwrap();
        assert_eq!(*env.get("x").unwrap(), 10.0);
    }
}
