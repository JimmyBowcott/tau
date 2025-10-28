use std::collections::HashMap;

use super::units::Dimension;

#[derive(Debug)]
pub struct Var {
    defined: bool,
    unit: Dimension,
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

    pub fn declare(&mut self, name: &str, unit: Dimension) -> Result<(), String> {
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
            Err(format!("Variable '{}' used before declaration", name))
        }
    }
}
