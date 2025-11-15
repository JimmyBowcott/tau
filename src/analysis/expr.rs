use crate::ast::{BinaryOp, Expr, UnitExpr};
use super::{units::Unit, Analyser};


impl Expr {
    pub fn validate(&self, ctx: &mut Analyser) -> Result<(), String> {
        self.check_declared(ctx)?;
        self.check_unit(ctx)?;
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

    fn check_unit(&self, ctx: &mut Analyser) -> Result<Unit, String> {
        match self {
            Expr::Number(_) => Ok(Unit::new([0; 7])),

            Expr::Identifier(name) => ctx
                .symbols
                .get_unit(name)
                .ok_or_else(|| format!("Undeclared variable '{}'", name)),

            Expr::Binary { left, op, right } => {
                let ldim = left.check_unit(ctx)?;
                let rdim = right.check_unit(ctx)?;

                match op {
                    BinaryOp::Add | BinaryOp::Subtract => {
                        if ldim == rdim {
                            Ok(ldim)
                        } else {
                            Err(format!(
                                "Unit mismatch: cannot {} {} and {}",
                                op.to_string(),
                                ldim,
                                rdim,
                            ))
                        }
                    }

                    BinaryOp::Multiply => Ok(ldim.add(&rdim)),
                    BinaryOp::Divide => Ok(ldim.sub(&rdim)),
                    BinaryOp::Power => match **right {
                        Expr::Number(exp_val) => {
                            if (exp_val.fract()).abs() > std::f64::EPSILON {
                                return Err(format!(
                                    "Non-integer exponent {} not allowed for units",
                                    exp_val
                                ));
                            }
                            let n = exp_val as i32;
                            Ok(ldim.mul(n as f64))
                        }
                        _ => Err("Exponent must be a dimensionless numeric literal".into()),
                    },
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

#[cfg(test)]
mod tests {
    use crate::ast::{BinaryOp, Expr};
    use crate::analysis::{Analyser, units::Unit};

    fn setup_analyser() -> Analyser {
        let mut analyser = Analyser::new();
        analyser.symbols.declare("mass", Unit::new([1, 0, 0, 0, 0, 0, 0])).unwrap();      // kg
        analyser.symbols.define("mass");
        analyser.symbols.declare("accel", Unit::new([0, 1, -2, 0, 0, 0, 0])).unwrap();    // m·s⁻²
        analyser.symbols.define("accel");
        analyser.symbols.declare("force", Unit::new([1, 1, -2, 0, 0, 0, 0])).unwrap();    // kg·m·s⁻² (N)
        analyser.symbols.define("force");
        analyser.symbols.declare("pressure", Unit::new([1, -1, -2, 0, 0, 0, 0])).unwrap(); // kg·m⁻¹·s⁻² (Pa)
        analyser.symbols.define("pressure");
        analyser.symbols.declare("area", Unit::new([0, 2, 0, 0, 0, 0, 0])).unwrap();       // m²
        analyser.symbols.define("area");
        analyser
    }

    #[test]
    fn number_is_dimensionless() {
        let expr = Expr::Number(9.81);
        let mut ctx = setup_analyser();
        assert_eq!(expr.check_unit(&mut ctx).unwrap(), Unit::new([0; 7]));
    }

    #[test]
    fn variable_dimension_matches_declaration() {
        let expr = Expr::Identifier("mass".into());
        let mut ctx = setup_analyser();
        assert_eq!(expr.check_unit(&mut ctx).unwrap(), Unit::new([1, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn addition_same_dimension_ok() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("force".into())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Identifier("force".into())),
        };
        let mut ctx = setup_analyser();
        assert_eq!(expr.check_unit(&mut ctx).unwrap(), Unit::new([1, 1, -2, 0, 0, 0, 0]));
    }

    #[test]
    fn addition_mismatched_units_fails() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("pressure".into())),
            op: BinaryOp::Add,
            right: Box::new(Expr::Identifier("force".into())),
        };
        let mut ctx = setup_analyser();
        let err = expr.check_unit(&mut ctx).unwrap_err();
        assert!(err.contains("Unit mismatch"));
    }

    #[test]
    fn multiplication_combines_dimensions() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("mass".into())),
            op: BinaryOp::Multiply,
            right: Box::new(Expr::Identifier("accel".into())),
        };
        let mut ctx = setup_analyser();
        assert_eq!(expr.check_unit(&mut ctx).unwrap(), Unit::new([1, 1, -2, 0, 0, 0, 0]));
    }

    #[test]
    fn division_subtracts_dimensions() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("force".into())),
            op: BinaryOp::Divide,
            right: Box::new(Expr::Identifier("area".into())),
        };
        let mut ctx = setup_analyser();
        assert_eq!(expr.check_unit(&mut ctx).unwrap(), Unit::new([1, -1, -2, 0, 0, 0, 0]));
    }

    #[test]
    fn power_integer_exponent_valid() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("area".into())),
            op: BinaryOp::Power,
            right: Box::new(Expr::Number(2.0)),
        };
        let mut ctx = setup_analyser();
        assert_eq!(expr.check_unit(&mut ctx).unwrap(), Unit::new([0, 4, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn power_non_integer_exponent_fails() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Identifier("area".into())),
            op: BinaryOp::Power,
            right: Box::new(Expr::Number(0.5)),
        };
        let mut ctx = setup_analyser();
        let err = expr.check_unit(&mut ctx).unwrap_err();
        assert!(err.contains("Non-integer exponent"));
    }
}
