use crate::number_utils::digit_dist::DigitDistribution;
use crate::primes::Primes;

pub fn ans() -> u128 {
    let mut primes: Primes = Primes::default();

    let is_prime = |x: &u128| primes.is_prime(*x);

    (10..1_000_000_000u128)
        .rev()
        .step_by(2)
        .filter(is_prime)
        .find(is_pandigital)
        .unwrap()
}

fn is_pandigital(n: &u128) -> bool {
    let digit_distribution = DigitDistribution::from(*n);
    if digit_distribution[0] != 0 {
        false
    } else {
        digit_distribution.get_distribution()[1..]
            .windows(2)
            .all(|pair| !is_rising_pair(pair) && pair[0] < 2)
    }
}

fn is_rising_pair(pair: &[usize]) -> bool{
    pair[1] > pair[0]
}

#[cfg(test)]
mod test;
