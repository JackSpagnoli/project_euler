use super::*;

#[test]
fn test_prime_factor_exponents() {
    let cases = [
        (1, vec![0; 25]),
        (2, [vec![1], vec![0; 24]].concat()),
        (3, [vec![0], vec![1], vec![0; 23]].concat()),
        (4, [vec![2], vec![0; 24]].concat()),
        (12, [vec![2], vec![1], vec![0; 23]].concat()),
    ];

    cases
        .iter()
        .for_each(|(n, factors)| assert_eq!(&get_prime_factor_exponents(n).to_vec(), factors));
}

#[test]
fn test_are_coprime() {
    let cases = [
        (vec![1], true),
        (vec![1, 2], true),
        (vec![2, 4], false),
        (vec![3, 3], false),
        (vec![6, 12, 24], false),
        (vec![37, 16], true),
    ];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(are_coprime(input.to_vec()), *expected));
}

#[test]
fn test_root_coprime() {
    let cases = [
        (1, true),
        (2, true),
        (3, true),
        (4, false),
        (6, true),
        (12, true),
        (36, false),
    ];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(is_root_coprime(input), *expected));
}

#[test]
fn test_max_digital_sum_for_a() {
    let cases = [(1, 1), (2, 151)];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(max_digital_sum_for_a(*input), *expected));
}
