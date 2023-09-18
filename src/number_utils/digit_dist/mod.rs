use super::digits::Digits;
use std::ops::Index;

pub struct DigitDistribution {
    distribution: [usize; 10],
}

impl DigitDistribution{
    pub fn get_distribution(self) -> [usize; 10]{
        self.distribution
    }
}

impl Index<usize> for DigitDistribution {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.distribution[index]
    }
}

impl From<u128> for DigitDistribution {
    fn from(value: u128) -> Self {
        let distribution = Digits::from(value).fold([0; 10], |mut distribution, digit| {
            distribution[digit] += 1;
            distribution
        });
        Self { distribution }
    }
}

#[cfg(test)]
mod test;
