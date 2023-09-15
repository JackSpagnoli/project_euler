use self::spiral_corner_prime_ratio::PrimeRatio;

pub fn ans() -> u128 {
    PrimeRatio::default()
        .find(|((numerator, denominator), _)| denominator >= &(10 * numerator))
        .unwrap()
        .1 as u128
}

mod spiral_corner_prime_ratio;
mod spiral_corners;

#[cfg(test)]
mod tests;
