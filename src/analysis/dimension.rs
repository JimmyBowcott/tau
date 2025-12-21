use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Dimension {
    // Represented as [kg, m, s, A, K, mol, cd]
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

    pub fn mul(&self, n: f64) -> Self {
        let mut result = [0; 7];
        for i in 0..7 {
            result[i] = (self.exponents[i] as f64 * n) as i8;
        }
        Self::new(result)
    }

    pub fn is_dimensionless(&self) -> bool {
        self.exponents.iter().all(|e| *e == 0)
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = ["kg", "m", "s", "A", "K", "mol", "cd"];

        if self.is_dimensionless() {
            return write!(f, "dimensionless");
        }

        let res: Vec<String> = self
            .exponents
            .iter()
            .zip(names.iter())
            .filter_map(|(&exp, &name)| {
                if exp == 0 {
                    None
                } else if exp == 1 {
                    Some(name.to_string())
                } else {
                    Some(format!("{}^{}", name, exp))
                }
            })
            .collect();

        write!(f, "{}", res.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::Dimension;

    #[test]
    fn test_unit_display() {
        let expected = "kg.m^2.s^3.A^4.K^5.mol^6.cd^7";
        let actual = Dimension::new([1, 2, 3, 4, 5, 6, 7]).to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add() {
        let dim_1 = Dimension::new([1, 2, 3, -4, 5, 6, 7]);
        let dim_2 = Dimension::new([7, 6, 5, 4, 3, 2, 1]);
        let expected = Dimension::new([8, 8, 8, 0, 8, 8, 8]);
        let actual = dim_1.add(&dim_2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub() {
        let dim_1 = Dimension::new([0, 1, 1, -2, 0, 0, 0]);
        let dim_2 = Dimension::new([0, 0, 2, -1, 1, 0, 0]);
        let expected = Dimension::new([0, 1, -1, -1, -1, 0, 0]);
        let actual = dim_1.sub(&dim_2);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_powr() {
        let dim_1 = Dimension::new([1, 1, 2, 0, 0, 0, 0]);
        let expected = Dimension::new([2, 2, 4, 0, 0, 0, 0]);
        let actual = dim_1.mul(2.0);
        assert_eq!(expected, actual);
    }
}
