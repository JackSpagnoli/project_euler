use fraction::DynaFraction as Fraction;

use super::{iterate, num_of_digits, numerator_more_digits_than_denominator, powerful_numer_count};

#[test]
fn test_iterate() {
    let init: Fraction<u128> = Fraction::from(1);

    let expected: [Fraction<u128>; 8] = [
        Fraction::from(3) / Fraction::from(2),
        Fraction::from(7) / Fraction::from(5),
        Fraction::from(17) / Fraction::from(12),
        Fraction::from(41) / Fraction::from(29),
        Fraction::from(99) / Fraction::from(70),
        Fraction::from(239) / Fraction::from(169),
        Fraction::from(577) / Fraction::from(408),
        Fraction::from(1393) / Fraction::from(985),
    ];

    let _: Vec<i32> = expected
        .iter()
        .scan(init, |state, expected| {
            *state = iterate(state);
            assert_eq!(state, expected);
            Some(0)
        })
        .collect();
}

#[test]
fn test_num_digits() {
    let cases = [(1, 1), (2, 1), (10, 2), (45, 2), (105, 3), (200, 3)];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(num_of_digits(*input), *expected))
}

#[test]
fn test_numer_digits() {
    let cases = [
        (Fraction::from(3) / Fraction::from(2), false),
        (Fraction::from(7) / Fraction::from(5), false),
        (Fraction::from(17) / Fraction::from(12), false),
        (Fraction::from(1393) / Fraction::from(985), true),
    ];

    cases.iter().for_each(|(input, expected)| {
        assert_eq!(numerator_more_digits_than_denominator(input), *expected)
    })
}

#[test]
fn test_count() {
    let cases = [(1, 0), (2, 0), (7, 0), (8, 1)];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(powerful_numer_count(*input), *expected))
}
