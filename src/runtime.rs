use std::collections::HashMap;

pub struct Env {
    pub vars: HashMap<String, f64>,
}

impl Env {
    pub fn new() -> Self {
        Self { vars: HashMap::new() }
    }
}
