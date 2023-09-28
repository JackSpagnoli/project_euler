use super::utils::is_bouncy;

pub struct BouncyNumberFraction {
    pub numerator: u128,
    pub denominator: u128,
    pub n: u128,
}

impl Iterator for BouncyNumberFraction {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        self.denominator += 1;
        self.n += 1;
        if is_bouncy(self.n) {
            self.numerator += 1;
        }
        Some(Self {
            numerator: self.numerator,
            denominator: self.denominator,
            n: self.n,
        })
    }
}

impl Default for BouncyNumberFraction {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 100,
            n: 100,
        }
    }
}
