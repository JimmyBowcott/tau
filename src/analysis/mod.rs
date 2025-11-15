mod symbols;
mod expr;
mod stmt;
mod dimension;
mod units;
pub use symbols::*;
use units::UnitTable;

use crate::ast::Stmt;

pub struct Analyser {
    pub symbols: SymbolTable,
    pub units: UnitTable,
}

impl Analyser {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            units: UnitTable::new(),
        }
    }

    pub fn analyse(&mut self, stmts: &Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            stmt.analyse(self)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expr;

    fn stmt_let(name: &str, value: Expr) -> Stmt {
        Stmt::Let {
            name: name.to_string(),
            unit: None,
            value,
        }
    }

    #[test]
    fn analyser_accepts_simple_declarations() {
        // let a = 1; let b = a;
        let stmts = vec![
            stmt_let("a", Expr::Number(1.0)),
            stmt_let("b", Expr::Identifier("a".into())),
        ];

        let mut ctx = Analyser::new();
        assert!(ctx.analyse(&stmts).is_ok());
    }

    #[test]
    fn analyser_rejects_use_before_declaration() {
        // let a = b; let b = 2;
        let stmts = vec![
            stmt_let("a", Expr::Identifier("b".into())),
            stmt_let("b", Expr::Number(2.0)),
        ];

        let mut ctx = Analyser::new();
        let res = ctx.analyse(&stmts);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Variable 'b' used before declaration");
    }

    #[test]
    fn analyser_rejects_redeclaration() {
        // let a = 1; let a = 2;
        let stmts = vec![
            stmt_let("a", Expr::Number(1.0)),
            stmt_let("a", Expr::Number(2.0)),
        ];

        let mut ctx = Analyser::new();
        let res = ctx.analyse(&stmts);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Variable 'a' already declared");
    }
}
