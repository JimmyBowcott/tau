use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::ast::{UnitExpr, UnitOp};
use super::{dimension::Dimension, Analyser};

impl Analyser {
    pub fn get_unit(&self, expr: &UnitExpr) -> Result<Unit, String> {
        match expr {
            UnitExpr::Symbol(name) => {
                let unit = self.units.get_unit(name);
                match unit {
                    Some(u) => Ok(u),
                    None => Err(format!("Unknown unit {}", name)),
                }
            }
            UnitExpr::Binary { left, op, right } => {
                let l = self.get_unit(left)?;
                let r = self.get_unit(right)?;
                Ok(match op {
                    UnitOp::Multiply => l.add(&r),
                    UnitOp::Divide => l.sub(&r),
                })
            }
            UnitExpr::Power { base, exponent } => {
                let b = self.get_unit(base)?;
                Ok(b.mul(*exponent))
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Unit {
    allows_prefix: bool,
    dimension: Dimension,
    scale: f64,
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
        self.dimension.is_dimensionless()
    }

    pub fn add(&self, other: &Unit) -> Self {
        let dimension = self.dimension.add(&other.dimension);

        Unit {
            dimension,
            allows_prefix: self.allows_prefix,
            scale: self.scale,
        }
    }

    pub fn sub(&self, other: &Unit) -> Self {
        let dimension = self.dimension.sub(&other.dimension);

        Unit {
            dimension,
            allows_prefix: self.allows_prefix,
            scale: self.scale,
        }
    }

    pub fn mul(&self, n: f64) -> Self {
        let dimension = self.dimension.mul(n);

        Unit {
            dimension,
            allows_prefix: self.allows_prefix,
            scale: self.scale,
        }
    }
}

impl PartialEq for Unit {
    fn eq(&self, rhs: &Unit) -> bool {
        self.scale == rhs.scale && self.dimension == rhs.dimension
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dimension.fmt(f)
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

    pub fn get_unit(&self, name: &str) -> Option<Unit> {
        match self.name_to_base_components.get(name) {
            Some(unit) => Some(unit.clone()),
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

    #[test]
    fn test_symbol() {
        let ctx = Analyser::new();
        let expr = UnitExpr::Symbol("m".to_string());
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 1, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_multiply() {
        let ctx = Analyser::new();
        let expr = UnitExpr::Binary {
            left: Box::new(UnitExpr::Symbol("A".to_string())),
            op: UnitOp::Multiply,
            right: Box::new(UnitExpr::Symbol("s".to_string())),
        };
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 0, 1, 1, 0, 0, 0]));
    }

    #[test]
    fn test_divide() {
        let ctx = Analyser::new();
        let expr = UnitExpr::Binary {
            left: Box::new(UnitExpr::Symbol("m".to_string())),
            op: UnitOp::Divide,
            right: Box::new(UnitExpr::Symbol("s".to_string())),
        };
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 1, -1, 0, 0, 0, 0]));
    }

    #[test]
    fn test_power() {
        let ctx = Analyser::new();
        let expr = UnitExpr::Power {
            base: Box::new(UnitExpr::Symbol("cd".to_string())),
            exponent: 2.0,
        };
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 0, 0, 0, 0, 0, 2]));
    }

    #[test]
    fn test_derived_symbol() {
        let ctx = Analyser::new();
        let expr = UnitExpr::Symbol("N".to_string());
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([1, 1, -2, 0, 0, 0, 0]));
    }

    #[test]
    fn test_complex_expr() {
        let ctx = Analyser::new();
        // m^2 / s^2
        let expr = UnitExpr::Binary {
            left: Box::new(UnitExpr::Power {
                base: Box::new(UnitExpr::Symbol("N".to_string())),
                exponent: 2.0,
            }),
            op: UnitOp::Multiply,
            right: Box::new(UnitExpr::Power {
                base: Box::new(UnitExpr::Symbol("m".to_string())),
                exponent: 2.0,
            }),
        };
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([2, 4, -4, 0, 0, 0, 0]));
    }

    #[test]
    fn test_unknown_unit() {
        let ctx = Analyser::new();
        let expr = UnitExpr::Symbol("foo".to_string());
        let res = ctx.get_unit(&expr);
        assert!(res.is_err());
    }
}
