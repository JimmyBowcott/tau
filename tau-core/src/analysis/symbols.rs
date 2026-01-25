use std::collections::HashMap;

use super::units::Unit;

#[derive(Debug)]
pub struct Var {
    defined: bool,
    unit: Unit,
    mutable: bool,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    vars: HashMap<String, Var>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get_unit(&self, name: &str) -> Option<Unit> {
        self.vars.get(name).map(|v| v.unit.clone())
    }

    pub fn declare(&mut self, name: &str, unit: Unit, mutable: bool) -> Result<(), String> {
        if self.vars.contains_key(name) {
            return Err(format!("Variable '{}' already declared", name));
        }
        self.vars.insert(
            name.to_string(),
            Var {
                defined: false,
                unit,
                mutable,
            },
        );
        Ok(())
    }

    pub fn define(&mut self, name: &str) {
        if let Some(flag) = self.vars.get_mut(name) {
            flag.defined = true;
        }
    }

    pub fn check_declared(&self, name: &str) -> Result<(), String> {
        if self.vars.contains_key(name) {
            Ok(())
        } else {
            Err(format!("Undeclared variable '{}'", name))
        }
    }

    pub fn check_assigned(&self, name: &str) -> Result<(), String> {
        let val = self.vars.get(name).ok_or("Undeclared variable '{}'")?;
        if val.defined {
            Ok(())
        } else {
            Err(format!("Variable {} declared but not assigned", name))
        }
    }

    pub fn assign(&mut self, name: &str, unit: Unit) -> Result<(), String> {
        self.check_declared(name)?;
        self.check_mutable(name)?;
        self.check_unit(name, unit)?;
        Ok(())
    }

    fn check_mutable(&self, name: &str) -> Result<(), String> {
        if let Some(var) = self.vars.get(name) {
            if var.mutable {
                return Ok(())
            } else {
                return Err(format!(
                    "{} is a constant, use 'let' instead of 'const' to make it mutable",
                    name
                ))
            }
        } else {
            return Err(format!("Undeclared variable '{}'", name))
        }
    }

    fn check_unit(&self, name: &str, unit: Unit) -> Result<(), String> {
        if let Some(var) = self.vars.get(name) {
            if var.unit == unit {
                return Ok(())
            } else {
                return Err(format!(
                    "Expected {}, found {}",
                    var.unit,
                    unit,
                ))
            }
        } else {
            return Err(format!("Undeclared variable '{}'", name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Unit;

    fn unit(dim: [i8; 7]) -> Unit {
        Unit::new(dim)
    }

    #[test]
    fn declare_and_get_unit() {
        let mut table = SymbolTable::new();
        let u = unit([1,0,0,0,0,0,0]);
        assert!(table.declare("x", u.clone(), true).is_ok());
        assert_eq!(table.get_unit("x"), Some(u.clone()));
    }

    #[test]
    fn redeclare_variable_fails() {
        let mut table = SymbolTable::new();
        let u = unit([0; 7]);
        assert!(table.declare("x", u.clone(), true).is_ok());
        let err = table.declare("x", u.clone(), true).unwrap_err();
        assert!(err.contains("already declared"));
    }

    #[test]
    fn define_marks_defined() {
        let mut table = SymbolTable::new();
        let u = unit([0; 7]);
        table.declare("x", u.clone(), true).unwrap();
        table.define("x");
        let var = table.vars.get("x").unwrap();
        assert!(var.defined);
    }

    #[test]
    fn check_declared() {
        let mut table = SymbolTable::new();
        table.declare("x", unit([0; 7]), true).unwrap();
        assert!(table.check_declared("x").is_ok());
    }

    #[test]
    fn check_undeclared() {
        let table = SymbolTable::new();
        let err = table.check_declared("y").unwrap_err();
        assert!(err.contains("Undeclared"));
    }

    #[test]
    fn assign_checks_unit() {
        let mut table = SymbolTable::new();
        let unit_1 = unit([1,0,0,0,0,0,0]);
        let unit_2 = unit([0,1,0,0,0,0,0]);
        table.declare("x", unit_1.clone(), true).unwrap();
        assert!(table.assign("x", unit_1.clone()).is_ok());
        let err = table.assign("x", unit_2.clone()).unwrap_err();
        assert!(err.contains("Expected"));
    }

    #[test]
    fn check_assign_to_undeclared() {
        let mut table = SymbolTable::new();
        let u = unit([0; 7]);
        let err = table.assign("y", u.clone()).unwrap_err();
        assert!(err.contains("Undeclared"));
    }

    #[test]
    fn check_assign_to_const() {
        let mut table = SymbolTable::new();
        let u = unit([0; 7]);
        table.declare("x", u.clone(), false).unwrap();
        table.define("x");
        let err = table.assign("x", u.clone()).unwrap_err();
        assert!(err.contains("constant"));
    }
}
