use crate::primes::Primes;

use super::spiral_corners::SpiralCorners;

pub struct PrimeRatio {
    spiral_corners: SpiralCorners,
    primes: Primes,
    numerator: u128,
    denominator: u128,
}

pub struct Ratio {
    pub numerator: u128,
    pub denominator: u128,
    pub side_length: usize,
}

impl Iterator for PrimeRatio {
    type Item = Ratio;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(corners) = self.spiral_corners.next() {
            self.numerator += corners
                .iter()
                .filter(|n| self.primes.is_prime(**n as u128))
                .count() as u128;
            self.denominator += 4;
            Some(Ratio {
                numerator: self.numerator,
                denominator: self.denominator,
                side_length: self.spiral_corners.side_length(),
            })
        } else {
            None
        }
    }
}

impl Default for PrimeRatio {
    fn default() -> Self {
        Self {
            spiral_corners: Default::default(),
            primes: Primes::default(),
            numerator: 0,
            denominator: 1,
        }
    }
}
