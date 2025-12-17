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
