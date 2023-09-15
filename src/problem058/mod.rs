use self::spiral_corner_prime_ratio::PrimeRatio;

pub fn ans() -> u128 {
    PrimeRatio::default()
        .find(ratio_less_than_ten_percent)
        .unwrap()
        .1 as u128
}

fn ratio_less_than_ten_percent(ratio_output: &((u128, u128), usize)) -> bool{
    let ((numerator, denominator), _) = ratio_output;
    denominator >= &(10 * numerator)
}

mod spiral_corner_prime_ratio;
mod spiral_corners;

#[cfg(test)]
mod tests;
