use std::collections::HashMap;
use crate::analysis::units::Dimension;

pub struct TypeEnv {
    vars: HashMap<String, Dimension>,
}

pub struct Env {
    vars: HashMap<String, f64>,
}

impl Env {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }
 
    pub fn insert(&mut self, name: String , value: f64) {
        self.vars.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&f64> {
        self.vars.get(name)
    }
}

impl TypeEnv {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }
 
    pub fn insert(&mut self, name: String , exponents: [i8; 7]) {
        self.vars.insert(name, Dimension::new(exponents));
    }

    pub fn get(&self, name: &str) -> Option<&Dimension> {
        self.vars.get(name)
    }
}
