use crate::ast::Stmt;

use super::Analyser;

impl Stmt {
    pub fn analyse(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            Stmt::Let { name, value, unit } => {
                if let Some(u) = unit {
                    u.validate(ctx)?;
                }

                let unit = match unit {
                    Some(u) => ctx.get_unit(u)?,
                    None => value.get_unit(ctx)?,
                };

                value.check_declared(ctx)?;
                value.assert_unit(ctx, &unit)?;
                ctx.symbols.declare(name, unit)?;
                ctx.symbols.define(name);
                Ok(())
            }
            Stmt::Print(expr) | Stmt::Expr(expr) => expr.validate(ctx),
        }
    }
}
