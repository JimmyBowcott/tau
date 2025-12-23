use super::{Analyser, units::Unit};
use crate::{
    ast::{BinaryOp, Expr, ExprKind, UnitExpr, UnitExprKind},
    error::Error,
};

impl Expr {
    pub fn validate(&self, ctx: &mut Analyser) -> Result<(), Error> {
        self.check_declared(ctx)?;
        self.check_assigned(ctx)?;
        self.check_unit_maths(ctx)?;
        Ok(())
    }

    pub fn check_declared(&self, ctx: &mut Analyser) -> Result<(), Error> {
        match &self.node {
            ExprKind::Number(_) => Ok(()),
            ExprKind::Identifier(name) => {
                ctx.symbols.check_declared(name).map_err(|e| self.error(e))
            }
            ExprKind::Binary { left, right, .. } => {
                left.check_declared(ctx)?;
                right.check_declared(ctx)
            }
        }
    }

    pub fn check_assigned(&self, ctx: &mut Analyser) -> Result<(), Error> {
        match &self.node {
            ExprKind::Number(_) => Ok(()),
            ExprKind::Identifier(name) => {
                ctx.symbols.check_assigned(name).map_err(|e| self.error(e))
            }
            ExprKind::Binary { left, right, .. } => {
                left.check_assigned(ctx)?;
                right.check_assigned(ctx)
            }
        }
    }

    pub fn assert_unit(&self, ctx: &mut Analyser, unit: &Unit) -> Result<(), Error> {
        let found_unit = self.get_unit(ctx)?;

        if !self.assert_equal(unit, &found_unit) {
            return Err(self.error(format!("Expected {}, found {}", unit, found_unit)));
        }

        Ok(())
    }

    pub fn get_unit(&self, ctx: &mut Analyser) -> Result<Unit, Error> {
        let dimensionless = Unit::dimensionless();
        match &self.node {
            ExprKind::Number(_) => Ok(Unit::dimensionless()),

            ExprKind::Identifier(name) => ctx
                .symbols
                .get_unit(name)
                .ok_or_else(|| self.error(format!("Undeclared variable '{}'", name))),

            ExprKind::Binary { left, op, right } => {
                let ldim = left.get_unit(ctx)?;
                let rdim = right.get_unit(ctx)?;

                match op {
                    BinaryOp::Add | BinaryOp::Subtract => {
                        if ldim == rdim || ldim == dimensionless || rdim == dimensionless {
                            Ok(ldim)
                        } else {
                            Err(self.error(format!(
                                "Unit mismatch: cannot {} {} and {}",
                                op.to_string(),
                                ldim,
                                rdim,
                            )))
                        }
                    }

                    BinaryOp::Multiply => Ok(ldim.add(&rdim)),
                    BinaryOp::Divide => Ok(ldim.sub(&rdim)),
                    BinaryOp::Power => {
                        match right.node {
                            ExprKind::Number(exp_val) => {
                                if (exp_val.fract()).abs() > std::f64::EPSILON {
                                    return Err(self.error(format!(
                                        "Non-integer exponent {} not allowed for units",
                                        exp_val
                                    )));
                                }
                                let n = exp_val as i32;
                                Ok(ldim.mul(n as f64))
                            }
                            _ => Err(self
                                .error("Exponent must be a dimensionless numeric literal)".into())),
                        }
                    }
                }
            }
        }
    }

    fn check_unit_maths(&self, ctx: &mut Analyser) -> Result<(), Error> {
        self.get_unit(ctx)?;
        Ok(())
    }

    fn assert_equal(&self, unit_1: &Unit, unit_2: &Unit) -> bool {
        let dimensionless = &Unit::dimensionless();

        if unit_1 == unit_2 || unit_1 == dimensionless || unit_2 == dimensionless {
            true
        } else {
            false
        }
    }
}

impl UnitExpr {
    pub fn validate(&self, ctx: &mut Analyser) -> Result<(), Error> {
        match &self.node {
            UnitExprKind::Symbol(s) => ctx
                .units
                .validate(s)
                .map_err(|e| Error::new(self.line, self.column, e)),
            UnitExprKind::Power { base, .. } => base.validate(ctx),
            UnitExprKind::Binary { left, right, .. } => {
                left.validate(ctx)?;
                right.validate(ctx)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::analysis::{Analyser, units::Unit};
    use crate::ast::{BinaryOp, Expr, ExprKind};

    fn setup_analyser() -> Analyser {
        let mut analyser = Analyser::new();
        analyser
            .symbols
            .declare("mass", Unit::new([1, 0, 0, 0, 0, 0, 0]), false)
            .unwrap(); // kg
        analyser.symbols.define("mass");
        analyser
            .symbols
            .declare("accel", Unit::new([0, 1, -2, 0, 0, 0, 0]), false)
            .unwrap(); // m·s⁻²
        analyser.symbols.define("accel");
        analyser
            .symbols
            .declare("force", Unit::new([1, 1, -2, 0, 0, 0, 0]), false)
            .unwrap(); // kg·m·s⁻² (N)
        analyser.symbols.define("force");
        analyser
            .symbols
            .declare("pressure", Unit::new([1, -1, -2, 0, 0, 0, 0]), false)
            .unwrap(); // kg·m⁻¹·s⁻² (Pa)
        analyser.symbols.define("pressure");
        analyser
            .symbols
            .declare("area", Unit::new([0, 2, 0, 0, 0, 0, 0]), false)
            .unwrap(); // m²
        analyser.symbols.define("area");
        analyser
    }

    #[test]
    fn number_is_dimensionless() {
        let expr = Expr::new(ExprKind::Number(9.81), 0, 0);
        let mut ctx = setup_analyser();
        assert_eq!(expr.get_unit(&mut ctx).unwrap(), Unit::dimensionless());
    }

    #[test]
    fn variable_dimension_matches_declaration() {
        let expr = Expr::new(ExprKind::Identifier("mass".into()), 0, 0);
        let mut ctx = setup_analyser();
        assert_eq!(
            expr.get_unit(&mut ctx).unwrap(),
            Unit::new([1, 0, 0, 0, 0, 0, 0])
        );
    }

    #[test]
    fn addition_same_dimension_ok() {
        let expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Identifier("force".into()), 0, 0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::new(ExprKind::Identifier("force".into()), 0, 0)),
            },
            0,
            0,
        );
        let mut ctx = setup_analyser();
        assert_eq!(
            expr.get_unit(&mut ctx).unwrap(),
            Unit::new([1, 1, -2, 0, 0, 0, 0])
        );
    }

    #[test]
    fn addition_mismatched_units_fails() {
        let expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Identifier("pressure".into()), 0, 0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::new(ExprKind::Identifier("force".into()), 0, 0)),
            },
            0,
            0,
        );
        let mut ctx = setup_analyser();
        let err = expr.get_unit(&mut ctx).unwrap_err();
        assert!(err.message.contains("Unit mismatch"));
    }

    #[test]
    fn multiplication_combines_dimensions() {
        let expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Identifier("mass".into()), 0, 0)),
                op: BinaryOp::Multiply,
                right: Box::new(Expr::new(ExprKind::Identifier("accel".into()), 0, 0)),
            },
            0,
            0,
        );
        let mut ctx = setup_analyser();
        assert_eq!(
            expr.get_unit(&mut ctx).unwrap(),
            Unit::new([1, 1, -2, 0, 0, 0, 0])
        );
    }

    #[test]
    fn division_subtracts_dimensions() {
        let expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Identifier("force".into()), 0, 0)),
                op: BinaryOp::Divide,
                right: Box::new(Expr::new(ExprKind::Identifier("area".into()), 0, 0)),
            },
            0,
            0,
        );
        let mut ctx = setup_analyser();
        assert_eq!(
            expr.get_unit(&mut ctx).unwrap(),
            Unit::new([1, -1, -2, 0, 0, 0, 0])
        );
    }

    #[test]
    fn power_integer_exponent_valid() {
        let expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Identifier("area".into()), 0, 0)),
                op: BinaryOp::Power,
                right: Box::new(Expr::new(ExprKind::Number(2.0), 0, 0)),
            },
            0,
            0,
        );
        let mut ctx = setup_analyser();
        assert_eq!(
            expr.get_unit(&mut ctx).unwrap(),
            Unit::new([0, 4, 0, 0, 0, 0, 0])
        );
    }

    #[test]
    fn power_non_integer_exponent_fails() {
        let expr = Expr::new(
            ExprKind::Binary {
                left: Box::new(Expr::new(ExprKind::Identifier("area".into()), 0, 0)),
                op: BinaryOp::Power,
                right: Box::new(Expr::new(ExprKind::Number(0.5), 0, 0)),
            },
            0,
            0,
        );
        let mut ctx = setup_analyser();
        let err = expr.get_unit(&mut ctx).unwrap_err();
        assert!(err.message.contains("Non-integer exponent"));
    }
}
