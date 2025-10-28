use std::collections::{HashMap, HashSet};

use crate::ast::{UnitExpr, UnitOp};

use super::Analyser;

impl Analyser {
    pub fn get_dimension(&self, expr: &UnitExpr) -> Result<Dimension, String> {
        match expr {
            UnitExpr::Symbol(name) => {
                let unit = self.units.get_dimension(name);
                match unit {
                    Some(u) => Ok(u),
                    None => Err(format!("Unknown unit {}", name)),
                }
            }
            UnitExpr::Binary { left, op, right } => {
                let l = self.get_dimension(left)?;
                let r = self.get_dimension(right)?;
                Ok(match op {
                    UnitOp::Multiply => l.add(&r),
                    UnitOp::Divide => l.sub(&r),
                })
            }
            UnitExpr::Power { base, exponent } => {
                let b = self.get_dimension(base)?;
                Ok(b.scale(*exponent))
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Dimension {
    pub exponents: [i8; 7],
}

impl Dimension {
    pub fn new(exponents: [i8; 7]) -> Self {
        Self { exponents }
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
}

#[derive(Clone)]
pub struct Unit {
    /// Represented as [kg, m, s, A, K, mol, cd]
    pub dimension: Dimension,
    pub allows_prefix: bool,
    pub scale: f64,
}

impl Unit {
    pub fn new(exponents: [i8; 7]) -> Self {
        Self {
            dimension: Dimension::new(exponents),
            allows_prefix: true,
            scale: 1.0,
        }
    }

    pub fn with_prefix(exponents: [i8; 7], allows_prefix: bool) -> Self {
        Self {
            dimension: Dimension::new(exponents),
            allows_prefix,
            scale: 1.0,
        }
    }

    pub fn with_scale(exponents: [i8; 7], scale: f64) -> Self {
        Self {
            dimension: Dimension::new(exponents),
            allows_prefix: true,
            scale,
        }
    }

    pub fn base() -> Self {
        Self {
            dimension: Dimension::new([0; 7]),
            allows_prefix: false,
            scale: 1.0,
        }
    }

    pub fn is_dimensionless(&self) -> bool {
        self.dimension.exponents.iter().all(|&x| x == 0)
    }
}

pub struct UnitTable {
    name_to_base_components: HashMap<&'static str, Unit>,
    base_components_to_name: HashMap<[i8; 7], &'static str>,
    prefixes: HashSet<&'static str>,
}

impl UnitTable {
    pub fn new() -> Self {
        let mut name_to_base_components = HashMap::new();
        let mut base_components_to_name = HashMap::new();

        macro_rules! unit {
            ($name:expr, [$($e:expr),*]) => {{
                let dim = Unit::new([$($e),*]);
                name_to_base_components.insert($name, dim.clone());
                base_components_to_name.insert(dim.dimension.exponents, $name);
            }};
            ($name:expr, [$($e:expr),*], $allows_prefix:expr) => {{
                let dim = Unit::with_prefix([$($e),*], $allows_prefix);
                name_to_base_components.insert($name, dim.clone());
                base_components_to_name.insert(dim.exponents, $name);
            }};
        }

        unit!("m", [0, 1, 0, 0, 0, 0, 0]);
        unit!("s", [0, 0, 1, 0, 0, 0, 0]);
        unit!("A", [0, 0, 0, 1, 0, 0, 0]);
        unit!("K", [0, 0, 0, 0, 1, 0, 0]);
        unit!("mol", [0, 0, 0, 0, 0, 1, 0]);
        unit!("cd", [0, 0, 0, 0, 0, 0, 1]);

        let kg = Unit::with_prefix([1, 0, 0, 0, 0, 0, 0], false);
        name_to_base_components.insert("kg", kg.clone());
        base_components_to_name.insert(kg.dimension.exponents, "kg");

        let g = Unit::with_scale([1, 0, 0, 0, 0, 0, 0], 0.01);
        name_to_base_components.insert("g", g.clone());
        base_components_to_name.insert(g.dimension.exponents, "g");

        unit!("rad", [0, 0, 0, 0, 0, 0, 0]);
        unit!("sr", [0, 0, 0, 0, 0, 0, 0]);
        unit!("Hz", [0, 0, -1, 0, 0, 0, 0]);
        unit!("N", [1, 1, -2, 0, 0, 0, 0]);
        unit!("Pa", [1, -1, -2, 0, 0, 0, 0]);
        unit!("J", [1, 2, -2, 0, 0, 0, 0]);
        unit!("W", [1, 2, -3, 0, 0, 0, 0]);
        unit!("C", [0, 0, 1, 1, 0, 0, 0]);
        unit!("V", [1, 2, -3, -1, 0, 0, 0]);
        unit!("F", [-1, -2, 4, 2, 0, 0, 0]);
        unit!("Ω", [1, 2, -3, -2, 0, 0, 0]);
        unit!("S", [-1, -2, 3, 2, 0, 0, 0]);
        unit!("Wb", [1, 2, -2, -1, 0, 0, 0]);
        unit!("T", [1, 0, -2, -1, 0, 0, 0]);
        unit!("H", [1, 2, -2, -2, 0, 0, 0]);
        unit!("lm", [0, 0, 0, 0, 0, 0, 1]);
        unit!("lx", [0, -2, 0, 0, 0, 0, 1]);
        unit!("Bq", [0, 0, -1, 0, 0, 0, 0]);
        unit!("Gy", [0, 2, -2, 0, 0, 0, 0]);
        unit!("Sv", [0, 2, -2, 0, 0, 0, 0]);
        unit!("kat", [0, 0, -1, 0, 0, 1, 0]);

        let prefixes: HashSet<&'static str> = [
            "Y", "Z", "E", "P", "T", "G", "M", "k", "h", "da", "d", "c", "m", "u", "µ", "n", "p",
            "f", "a", "z", "y",
        ]
        .into_iter()
        .collect();

        Self {
            name_to_base_components,
            base_components_to_name,
            prefixes,
        }
    }

    pub fn simplify(&self, dim: &Unit) -> &'static str {
        self.base_components_to_name
            .get(&dim.dimension.exponents)
            .copied()
            .unwrap_or("unknown")
    }

    pub fn validate(&self, value: &str) -> Result<(), String> {
        if self.name_to_base_components.contains_key(value) {
            return Ok(());
        }

        if let Some((prefix, base)) = self.split_prefix(value) {
            let unit_info = self
                .name_to_base_components
                .get(base)
                .ok_or_else(|| format!("Unknown unit '{}'", base))?;

            if unit_info.allows_prefix {
                Ok(())
            } else {
                Err(format!("Prefix '{}' not allowed for '{}'", prefix, base))
            }
        } else {
            Err(format!("Unknown unit '{}'", value))
        }
    }

    pub fn get_dimension(&self, name: &str) -> Option<Dimension> {
        match self.name_to_base_components.get(name) {
            Some(unit) => Some(unit.dimension.clone()),
            None => None,
        }
    }

    fn split_prefix<'a>(&self, value: &'a str) -> Option<(&'a str, &'a str)> {
        self.prefixes
            .iter()
            .find_map(|prefix| value.strip_prefix(prefix).map(|rest| (*prefix, rest)))
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
    fn validates_valid_combinations() {
        let analyzer = UnitTable::new();
        assert!(analyzer.validate(&"mg".to_string()).is_ok());
        assert!(analyzer.validate(&"MJ".to_string()).is_ok());
        assert!(analyzer.validate(&"kN".to_string()).is_ok());
        assert!(analyzer.validate(&"GPa".to_string()).is_ok());
        assert!(analyzer.validate(&"ms".to_string()).is_ok());
    }

    #[test]
    fn does_not_validate_invalid_combinations() {
        let analyzer = UnitTable::new();
        assert!(analyzer.validate(&"kkg".to_string()).is_err());
        assert!(analyzer.validate(&"ukg".to_string()).is_err());
    }

    #[test]
    fn rejects_invalid_units() {
        let analyzer = UnitTable::new();
        assert!(analyzer.validate(&"lightyear".to_string()).is_err());
        assert!(analyzer.validate(&"wat".to_string()).is_err());
    }
}
