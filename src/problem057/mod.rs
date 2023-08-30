use fraction::{BigUint, DynaFraction as Fraction};

pub fn ans() -> u128 {
    powerful_numer_count(1000)
}

fn powerful_numer_count(terms: usize) -> u128 {
    let initial = Fraction::from(1);
    (0..terms)
        .scan(initial, |state, _| {
            *state = iterate(state);
            Some(numerator_more_digits_than_denominator(state))
        })
        .filter(|x| *x)
        .count() as u128
}

fn numerator_more_digits_than_denominator(fraction: &Fraction<u128>) -> bool {
    match (
        fraction.numer().unwrap().clone().unpack(),
        fraction.denom().unwrap().clone().unpack(),
    ) {
        (Ok(numer), Ok(denom)) => num_of_digits(numer) > num_of_digits(denom),
        (Err(numer), Err(denom)) => num_of_digits_big(numer) > num_of_digits_big(denom),
        _ => panic!("hfdskfkds"),
    }
}

fn num_of_digits(num: u128) -> usize {
    let mut digits = 1;
    let mut num = num;
    while num >= 10 {
        digits += 1;
        num /= 10;
    }
    digits
}

fn num_of_digits_big(num: BigUint) -> usize {
    let mut digits = 1;
    let mut num = num;
    while num >= BigUint::from(10usize) {
        digits += 1;
        num /= BigUint::from(10usize);
    }
    digits
}

fn iterate(previous: &Fraction<u128>) -> Fraction<u128> {
    Fraction::from(1) + (Fraction::from(1) / (Fraction::from(1) + previous.clone()))
}

#[cfg(test)]
mod test;
