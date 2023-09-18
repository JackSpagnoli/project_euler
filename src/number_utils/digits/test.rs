use super::Digits;

#[test]
fn test_digits() {
    let cases: [(u128, Vec<usize>); 7] = [
        (0, vec![0]),
        (1, vec![1]),
        (10, vec![0, 1]),
        (11, vec![1, 1]),
        (123, vec![3, 2, 1]),
        (101, vec![1, 0, 1]),
        (987654321, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ];

    cases.iter().for_each(|(n, expected)| {
        let digits = Digits::from(*n);

        assert!(digits.eq(expected.iter().copied()))
    })
}
