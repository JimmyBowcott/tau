use crate::{
    ast::{Stmt, StmtKind},
    error::Error,
};

use super::Analyser;

impl Stmt {
    pub fn analyse(&self, ctx: &mut Analyser) -> Result<(), Error> {
        match &self.node {
            StmtKind::Let { name, value, unit } |
                StmtKind::Const { name, value, unit } => {
                if let Some(u) = unit {
                    u.validate(ctx)?;
                }

                let unit = match unit {
                    Some(u) => ctx.get_unit(u)?,
                    None => value.get_unit(ctx)?,
                };

                value.check_declared(ctx)?;
                value.assert_unit(ctx, &unit)?;
                ctx.symbols
                    .declare(name, unit)
                    .map_err(|e| Error::new(self.line, self.column, e))?;
                ctx.symbols.define(name);
                Ok(())
            }
            StmtKind::Print(expr) | StmtKind::Expr(expr) => expr.validate(ctx),
        }
    }
}
