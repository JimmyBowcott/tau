use std::{collections::HashMap, fmt};

use super::{Analyser, dimension::Dimension};
use crate::{ast::{UnitExpr, UnitExprKind, UnitOp}, error::Error};

#[derive(Clone, Debug)]
pub struct Unit {
    allows_prefix: bool,
    dimension: Dimension,
    scale: f64,
}

struct UnitWithName {
    name: &'static str,
    exponents: [i8; 7],
    scale: f64,
    allows_prefix: bool,
}

pub struct UnitTable {
    name_to_base_components: HashMap<&'static str, Unit>,
    base_components_to_name: HashMap<[i8; 7], &'static str>,
    prefixes_to_scale: HashMap<&'static str, f64>,
}

const UNITS: &[UnitWithName] = &[
    UnitWithName {
        name: "kg",
        exponents: [1, 0, 0, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: false,
    },
    UnitWithName {
        name: "m",
        exponents: [0, 1, 0, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "s",
        exponents: [0, 0, 1, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "g",
        exponents: [1, 0, 0, 0, 0, 0, 0],
        scale: 0.01,
        allows_prefix: true,
    },
    UnitWithName {
        name: "A",
        exponents: [0, 0, 0, 1, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "K",
        exponents: [0, 0, 0, 0, 1, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "mol",
        exponents: [0, 0, 0, 0, 0, 1, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "cd",
        exponents: [0, 0, 0, 0, 0, 0, 1],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "rad",
        exponents: [0, 0, 0, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "sr",
        exponents: [0, 0, 0, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Hz",
        exponents: [0, 0, -1, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "N",
        exponents: [1, 1, -2, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Pa",
        exponents: [1, -1, -2, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "J",
        exponents: [1, 2, -2, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "W",
        exponents: [1, 2, -3, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "C",
        exponents: [0, 0, 1, 1, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "V",
        exponents: [1, 2, -3, -1, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "F",
        exponents: [-1, -2, 4, 2, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Ω",
        exponents: [1, 2, -3, -2, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "S",
        exponents: [-1, -2, 3, 2, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Wb",
        exponents: [1, 2, -2, -1, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "T",
        exponents: [1, 0, -2, -1, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "H",
        exponents: [1, 2, -2, -2, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "lm",
        exponents: [0, 0, 0, 0, 0, 0, 1],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "lx",
        exponents: [0, -2, 0, 0, 0, 0, 1],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Bq",
        exponents: [0, 0, -1, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Gy",
        exponents: [0, 2, -2, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "Sv",
        exponents: [0, 2, -2, 0, 0, 0, 0],
        scale: 1.0,
        allows_prefix: true,
    },
    UnitWithName {
        name: "kat",
        exponents: [0, 0, -1, 0, 0, 1, 0],
        scale: 1.0,
        allows_prefix: true,
    },
];

const PREFIXES: &[(&'static str, f64)] = &[
    ("Y", 1e24),
    ("Z", 1e21),
    ("E", 1e18),
    ("P", 1e15),
    ("T", 1e12),
    ("G", 1e9),
    ("M", 1e6),
    ("k", 1e3),
    ("h", 1e2),
    ("da", 1e1),
    ("d", 1e-1),
    ("c", 1e-2),
    ("m", 1e-3),
    ("u", 1e-6),
    ("µ", 1e-6),
    ("n", 1e-9),
    ("p", 1e-12),
    ("f", 1e-15),
    ("a", 1e-18),
    ("z", 1e-21),
    ("y", 1e-24),
];

impl Analyser {
    pub fn get_unit(&self, expr: &UnitExpr) -> Result<Unit, Error> {
        match &expr.node {
            UnitExprKind::Symbol(name) => {
                let unit = self.units.get_unit(name);
                match unit {
                    Some(u) => Ok(u),
                    None => Err(Error::new(expr.line, expr.column, format!("Unknown unit {}", name))),
                }
            }
            UnitExprKind::Binary { left, op, right } => {
                let l = self.get_unit(left)?;
                let r = self.get_unit(right)?;
                Ok(match op {
                    UnitOp::Multiply => l.add(&r),
                    UnitOp::Divide => l.sub(&r),
                })
            }
            UnitExprKind::Power { base, exponent } => {
                let b = self.get_unit(base)?;
                Ok(b.mul(*exponent))
            }
        }
    }
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

    pub fn with_prefix_scale(exponents: [i8; 7], scale: f64, allows_prefix: bool) -> Self {
        Self {
            dimension: Dimension::new(exponents),
            allows_prefix,
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

impl UnitTable {
    pub fn new() -> Self {
        let mut name_to_base_components = HashMap::new();
        let mut base_components_to_name = HashMap::new();

        for def in UNITS {
            let unit = Unit::with_prefix_scale(def.exponents, def.scale, def.allows_prefix);

            name_to_base_components.insert(def.name, unit.clone());
            base_components_to_name.insert(def.exponents, def.name);
        }

        let prefixes_to_scale = PREFIXES.iter().cloned().collect();

        Self {
            name_to_base_components,
            base_components_to_name,
            prefixes_to_scale,
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
        self.prefixes_to_scale
            .keys()
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
        let expr = UnitExpr::new(UnitExprKind::Symbol("m".to_string()), 1, 1);
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 1, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_multiply() {
        let ctx = Analyser::new();
        let expr = UnitExpr::new(UnitExprKind::Binary {
            left: Box::new(UnitExpr::new(UnitExprKind::Symbol("A".to_string()), 1, 1)),
            op: UnitOp::Multiply,
            right: Box::new(UnitExpr::new(UnitExprKind::Symbol("s".to_string()), 1, 1)),
        }, 1, 1);
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 0, 1, 1, 0, 0, 0]));
    }

    #[test]
    fn test_divide() {
        let ctx = Analyser::new();
        let expr = UnitExpr::new(UnitExprKind::Binary {
            left: Box::new(UnitExpr::new(UnitExprKind::Symbol("m".to_string()), 1, 1)),
            op: UnitOp::Divide,
            right: Box::new(UnitExpr::new(UnitExprKind::Symbol("s".to_string()), 1, 1)),
        }, 1, 1);
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 1, -1, 0, 0, 0, 0]));
    }

    #[test]
    fn test_power() {
        let ctx = Analyser::new();
        let expr = UnitExpr::new(UnitExprKind::Power {
            base: Box::new(UnitExpr::new(UnitExprKind::Symbol("cd".to_string()), 1, 1)),
            exponent: 2.0,
        }, 1, 1);
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([0, 0, 0, 0, 0, 0, 2]));
    }

    #[test]
    fn test_derived_symbol() {
        let ctx = Analyser::new();
        let expr = UnitExpr::new(UnitExprKind::Symbol("N".to_string()), 1, 1);
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([1, 1, -2, 0, 0, 0, 0]));
    }

    #[test]
    fn test_complex_expr() {
        let ctx = Analyser::new();
        // m^2 / s^2
        let expr = UnitExpr::new(UnitExprKind::Binary {
            left: Box::new(UnitExpr::new(UnitExprKind::Power {
                base: Box::new(UnitExpr::new(UnitExprKind::Symbol("N".to_string()), 1, 1)),
                exponent: 2.0,
            }, 1, 1)),
            op: UnitOp::Multiply,
            right: Box::new(UnitExpr::new(UnitExprKind::Power {
                base: Box::new(UnitExpr::new(UnitExprKind::Symbol("m".to_string()), 1, 1)),
                exponent: 2.0,
            }, 1, 1)),
        }, 1, 1, );
        let unit = ctx.get_unit(&expr).unwrap();
        assert_eq!(unit, Unit::new([2, 4, -4, 0, 0, 0, 0]));
    }

    #[test]
    fn test_unknown_unit() {
        let ctx = Analyser::new();
        let expr = UnitExpr::new(UnitExprKind::Symbol("foo".to_string()), 1, 1);
        let res = ctx.get_unit(&expr);
        assert!(res.is_err());
    }
}
