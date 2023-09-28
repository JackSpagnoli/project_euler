use self::bouncy_number_fraction::BouncyNumberFraction;

pub fn ans() -> u128 {
    find_bouncy_number_percentage(99)
}

fn find_bouncy_number_percentage(k: u128) -> u128 {
    BouncyNumberFraction::default()
        .find(|fraction| fraction.numerator * 100 == fraction.denominator * k)
        .unwrap()
        .n
}

mod bouncy_number_fraction;
mod utils;

#[cfg(test)]
mod tests;
