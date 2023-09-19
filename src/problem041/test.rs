use super::{largest_pandigital_prime, number_from_digits};

#[test]
fn test_largest_pandigital_prime() {
    let cases = [(4, 4231)];

    cases
        .iter()
        .for_each(|(digits, expected)| assert_eq!(largest_pandigital_prime(*digits), *expected))
}

#[test]
fn test_number_from_digits() {
    let cases = [
        (vec![1], 1),
        (vec![1, 2], 12),
        (vec![1, 2, 3], 123),
        (vec![3, 2, 1], 321),
    ];

    cases
        .iter()
        .for_each(|(digits, expected)| assert_eq!(number_from_digits(digits.to_vec()), *expected))
}
