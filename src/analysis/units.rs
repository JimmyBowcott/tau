use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Dimension {
    /// Represented as [kg, m, s, A, K, mol, cd]
    pub exponents: [i8; 7],
}

impl Dimension {
    pub fn new(exponents: [i8; 7]) -> Self {
        Self { exponents }
    }

    pub fn base() -> Self {
        Self { exponents: [0; 7] }
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut result = [0; 7];
        for i in 0..7 {
            result[i] = self.exponents[i] + other.exponents[i];
        }
        Self::new(result)
    }

    pub fn sub(&self, other: &Self) -> Self {
        let mut result = [0; 7];
        for i in 0..7 {
            result[i] = self.exponents[i] - other.exponents[i];
        }
        Self::new(result)
    }

    pub fn scale(&self, n: f64) -> Self {
        let mut result = [0; 7];
        for i in 0..7 {
            result[i] = (self.exponents[i] as f64 * n) as i8;
        }
        Self::new(result)
    }

    pub fn is_dimensionless(&self) -> bool {
        self.exponents.iter().all(|&x| x == 0)
    }
}

pub struct UnitTable {
    base_units: HashSet<String>,
    derived_to_base: HashMap<String, Dimension>,
    base_to_derived: HashMap<[i8; 7], String>,
}

impl UnitTable {
    pub fn new() -> Self {
        let mut derived_to_base = HashMap::new();
        let mut base_to_derived = HashMap::new();

        macro_rules! unit {
            ($name:expr, [$($e:expr),*]) => {{
                let dim = Dimension::new([$($e),*]);
                derived_to_base.insert($name, dim.clone());
                base_to_derived.insert(dim.exponents, $name);
            }};
        }

        // Base units
        unit!("kg".to_string(), [1, 0, 0, 0, 0, 0, 0]);
        unit!("m".to_string(),  [0, 1, 0, 0, 0, 0, 0]);
        unit!("s".to_string(),  [0, 0, 1, 0, 0, 0, 0]);
        unit!("A".to_string(),  [0, 0, 0, 1, 0, 0, 0]);
        unit!("K".to_string(),  [0, 0, 0, 0, 1, 0, 0]);
        unit!("mol".to_string(),[0, 0, 0, 0, 0, 1, 0]);
        unit!("cd".to_string(), [0, 0, 0, 0, 0, 0, 1]);

        // Derived units
        unit!("N".to_string(),  [1, 1, -2, 0, 0, 0, 0]); // kg·m/s²
        unit!("J".to_string(),  [1, 2, -2, 0, 0, 0, 0]); // N·m
        unit!("W".to_string(),  [1, 2, -3, 0, 0, 0, 0]); // J/s
        unit!("Pa".to_string(), [1, -1, -2, 0, 0, 0, 0]); // N/m²
        unit!("C".to_string(),  [0, 0, 1, 1, 0, 0, 0]); // A·s
        unit!("V".to_string(),  [1, 2, -3, -1, 0, 0, 0]); // W/A
        unit!("Ω".to_string(),  [1, 2, -3, -2, 0, 0, 0]); // V/A
                                                          //
        let valid_units = ["m", "s", "kg", "A", "K", "mol", "cd"];
        let mut base_units = HashSet::new();
        valid_units.iter().for_each(|unit| {
            base_units.insert(unit.to_string());
        });

        Self { base_units, derived_to_base, base_to_derived }
    }

    // pub fn simplify(&self, dim: &Dimension) -> &'static str {
    //     self.base_to_derived
    //         .get(&dim.exponents)
    //         .copied()
    //         .unwrap_or("unknown")
    // }

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
