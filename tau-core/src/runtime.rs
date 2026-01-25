use std::collections::HashMap;

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
