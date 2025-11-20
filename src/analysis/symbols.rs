use std::collections::HashMap;

use super::units::Unit;

#[derive(Debug)]
pub struct Var {
    defined: bool,
    unit: Unit,
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

    pub fn declare(&mut self, name: &str, unit: Unit) -> Result<(), String> {
        if self.vars.contains_key(name) {
            return Err(format!("Variable '{}' already declared", name));
        }
        self.vars.insert(
            name.to_string(),
            Var {
                defined: false,
                unit,
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
}
