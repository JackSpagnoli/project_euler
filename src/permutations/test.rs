use super::Permutations;

#[test]
fn test_permutations() {
    let cases = [
        (1, vec![vec![0]]),
        (2, vec![vec![0, 1], vec![1, 0]]),
        (
            3,
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 1, 0],
            ],
        ),
    ];

    cases.iter().for_each(|(n, expected)| {
        Permutations::from(*n)
            .zip(expected)
            .for_each(|(permutation, expected)| assert_eq!(permutation, expected.clone()))
    })
}
