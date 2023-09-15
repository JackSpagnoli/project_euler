use self::spiral_corner_prime_ratio::{PrimeRatio, Ratio};

pub fn ans() -> u128 {
    PrimeRatio::default()
        .find(ratio_less_than_ten_percent)
        .unwrap()
        .side_length as u128
}

fn ratio_less_than_ten_percent(ratio: &Ratio) -> bool {
    ratio.denominator >= (10 * ratio.numerator)
}

mod spiral_corner_prime_ratio;
mod spiral_corners;

#[cfg(test)]
mod tests;
