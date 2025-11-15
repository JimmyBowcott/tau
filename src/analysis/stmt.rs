use crate::ast::Stmt;

use super::{units::Unit, Analyser};

impl Stmt {
    pub fn analyse(&self, ctx: &mut Analyser) -> Result<(), String> {
        match self {
            Stmt::Let { name, value, unit } => {
                if let Some(u) = unit {
                    u.validate(ctx)?;
                }

                let unit = match unit {
                    Some(u) => ctx.get_unit(u)?,
                    None => Unit::new([0; 7]),
                };

                ctx.symbols.declare(name, unit)?;
                value.validate(ctx)?;
                ctx.symbols.define(name);
                Ok(())
            }
            Stmt::Print(expr) | Stmt::Expr(expr) => expr.validate(ctx),
        }
    }
}
