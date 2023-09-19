use crate::permutations::Permutations;
use crate::primes::Primes;
use std::convert::identity;

pub fn ans() -> u128 {
    largest_pandigital_prime(9)
}

fn largest_pandigital_prime(digits: usize) -> u128 {
    let mut primes: Primes = Primes::default();

    let is_prime = |x: &u128| primes.is_prime(*x);

    (1..=digits)
        .rev()
        .map(|digits| Permutations::from((1..=digits).collect::<Vec<usize>>()))
        .flat_map(identity)
        .map(number_from_digits)
        .filter(is_prime)
        .max()
        .unwrap()
}

fn number_from_digits(digits: Vec<usize>) -> u128 {
    digits
        .iter()
        .map(|x| *x as u128)
        .reduce(|sum, digit| (sum * 10) + digit)
        .unwrap()
}

#[cfg(test)]
mod test;
