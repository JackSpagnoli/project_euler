use super::DigitDistribution;

#[test]
fn test_digit_dist() {
    let cases = [
        (0, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        (1, [0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
        (2, [0, 0, 1, 0, 0, 0, 0, 0, 0, 0]),
        (15, [0, 1, 0, 0, 0, 1, 0, 0, 0, 0]),
        (23, [0, 0, 1, 1, 0, 0, 0, 0, 0, 0]),
        (104, [1, 1, 0, 0, 1, 0, 0, 0, 0, 0]),
        (222, [0, 0, 3, 0, 0, 0, 0, 0, 0, 0]),
    ];

    cases.iter().for_each(|(n, expected)| {
        let distribution = DigitDistribution::from(*n);
        (0..10).for_each(|i| assert_eq!(distribution[i], expected[i]));
    })
}
