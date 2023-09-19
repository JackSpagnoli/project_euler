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

#[test]
fn test_permutations_generic() {
    let cases = [
        (vec!['a'], vec![vec!['a']]),
        (vec!['a', 'b'], vec![vec!['a', 'b'], vec!['b', 'a']]),
        (
            vec!['a', 'b', 'c'],
            vec![
                vec!['a', 'b', 'c'],
                vec!['b', 'a', 'c'],
                vec!['c', 'a', 'b'],
                vec!['a', 'c', 'b'],
                vec!['b', 'c', 'a'],
                vec!['c', 'b', 'a'],
            ],
        ),
    ];

    cases.iter().for_each(|(n, expected)| {
        Permutations::from(n.to_owned())
            .zip(expected)
            .for_each(|(permutation, expected)| assert_eq!(permutation, expected.clone()))
    })
}
