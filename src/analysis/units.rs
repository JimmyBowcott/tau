use std::collections::HashSet;

pub struct UnitTable {
    base_units: HashSet<String>,
}

impl UnitTable {
    pub fn new() -> Self {
        let valid_units = ["m", "s", "kg", "A", "K", "mol", "cd"];
        let mut base_units = HashSet::new();
        valid_units.iter().for_each(|unit| {
            base_units.insert(unit.to_string());
        });

        Self {
            base_units,
        }
    }

    pub fn validate_base(&self, value: &String) -> Result<(), String> {
        if self.base_units.contains(value) {
            Ok(())
        } else {
            Err(format!("Invalid SI unit: '{}'", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_known_units() {
        let analyzer = UnitTable::new();
        assert!(analyzer.validate_base(&"m".to_string()).is_ok());
        assert!(analyzer.validate_base(&"kg".to_string()).is_ok());
        assert!(analyzer.validate_base(&"s".to_string()).is_ok());
    }

    #[test]
    fn rejects_invalid_units() {
        let analyzer = UnitTable::new();
        assert!(analyzer.validate_base(&"lightyear".to_string()).is_err());
        assert!(analyzer.validate_base(&"wat".to_string()).is_err());
    }
}
