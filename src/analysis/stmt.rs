use crate::{
    ast::{Stmt, StmtKind},
    error::Error,
};

use super::Analyser;

impl Stmt {
    pub fn analyse(&self, ctx: &mut Analyser) -> Result<(), Error> {
        match &self.node {
            StmtKind::Let { .. } | StmtKind::Const { .. } => self.analyse_declaration(ctx),
            StmtKind::Assign { .. } => self.analyse_assign(ctx),
            StmtKind::Print(expr) | StmtKind::Expr(expr) => expr.validate(ctx),
        }
    }

    fn analyse_declaration(&self, ctx: &mut Analyser) -> Result<(), Error> {
        let (name, unit, value, mutable) = match &self.node {
            StmtKind::Let { name, unit, value } => (name, unit, value, true),
            StmtKind::Const { name, unit, value } => (name, unit, value, false),
            _ => unreachable!()
        };

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
            .declare(name, unit, mutable)
            .map_err(|e| Error::new(self.line, self.column, e))?;
        ctx.symbols.define(name);
        Ok(())
    }

    fn analyse_assign(&self, ctx: &mut Analyser) -> Result<(), Error> {
        let (name, value) = match &self.node {
            StmtKind::Assign { name, value } => (name, value),
            _ => unreachable!()
        };

        let unit = value.get_unit(ctx)?;
        ctx.symbols.assign(name, unit).map_err(|e| Error::new(self.line, self.column, e))?;
        Ok(())
    }
}
