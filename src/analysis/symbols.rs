use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SymbolTable {
    vars: HashMap<String, bool>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }

    pub fn declare(&mut self, name: &str) -> Result<(), String> {
        if self.vars.contains_key(name) {
            return Err(format!("Variable '{}' already declared", name));
        }
        self.vars.insert(name.to_string(), false);
        Ok(())
    }

    pub fn define(&mut self, name: &str) {
        if let Some(flag) = self.vars.get_mut(name) {
            *flag = true;
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

