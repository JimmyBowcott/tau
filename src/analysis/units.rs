use std::collections::HashMap;

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
    name_to_base_components: HashMap<&'static str, Dimension>,
    base_components_to_name: HashMap<[i8; 7], &'static str>,
}

impl UnitTable {
    pub fn new() -> Self {
        let mut name_to_base_components = HashMap::new();
        let mut base_components_to_name = HashMap::new();

        macro_rules! unit {
            ($name:expr, [$($e:expr),*]) => {{
                let dim = Dimension::new([$($e),*]);
                name_to_base_components.insert($name, dim.clone());
                base_components_to_name.insert(dim.exponents, $name);
            }};
        }

        // Base units
        unit!("kg", [1, 0, 0, 0, 0, 0, 0]);
        unit!("m",  [0, 1, 0, 0, 0, 0, 0]);
        unit!("s",  [0, 0, 1, 0, 0, 0, 0]);
        unit!("A",  [0, 0, 0, 1, 0, 0, 0]);
        unit!("K",  [0, 0, 0, 0, 1, 0, 0]);
        unit!("mol", [0, 0, 0, 0, 0, 1, 0]);
        unit!("cd", [0, 0, 0, 0, 0, 0, 1]);

        // Derived units
        unit!("N",  [1, 1, -2, 0, 0, 0, 0]); // kg·m/s²
        unit!("J",  [1, 2, -2, 0, 0, 0, 0]); // N·m
        unit!("W",  [1, 2, -3, 0, 0, 0, 0]); // J/s
        unit!("Pa", [1, -1, -2, 0, 0, 0, 0]); // N/m²
        unit!("C",  [0, 0, 1, 1, 0, 0, 0]); // A·s
        unit!("V",  [1, 2, -3, -1, 0, 0, 0]); // W/A
        unit!("Ω",  [1, 2, -3, -2, 0, 0, 0]); // V/A

        Self { name_to_base_components, base_components_to_name }
    }

    pub fn simplify(&self, dim: &Dimension) -> &'static str {
        self.base_components_to_name
            .get(&dim.exponents)
            .copied()
            .unwrap_or("unknown")
    }

    pub fn validate(&self, value: &String) -> Result<(), String> {
        if self.name_to_base_components.contains_key(value.as_str()) {
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
        assert!(analyzer.validate(&"m".to_string()).is_ok());
        assert!(analyzer.validate(&"kg".to_string()).is_ok());
        assert!(analyzer.validate(&"s".to_string()).is_ok());
        assert!(analyzer.validate(&"N".to_string()).is_ok());
        assert!(analyzer.validate(&"Pa".to_string()).is_ok());
    }

    #[test]
    fn rejects_invalid_units() {
        let analyzer = UnitTable::new();
        assert!(analyzer.validate(&"lightyear".to_string()).is_err());
        assert!(analyzer.validate(&"wat".to_string()).is_err());
    }
}
