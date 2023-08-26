use super::*;

#[test]
fn test_reverse() {
    let cases: [(u128, u128); 5] = [
        (1, 1),
        (12, 21),
        (123, 321),
        (3478923432, 2343298743),
        (1067869, 9687601),
    ];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(reverse(*input), *expected))
}

#[test]
fn test_reverse_sum() {
    let cases: [(u128, u128); 4] = [(1, 2), (12, 33), (123, 444), (3478923432, 5822222175)];

    cases
        .iter()
        .for_each(|(input, expected)| assert_eq!(reverse_sum(*input), *expected))
}

#[test]
fn test_palindrome() {
    let cases = [
        (1, true),
        (11, true),
        (12, false),
        (123, false),
        (121, true),
        (34543, true),
    ];

    cases
        .iter()
        .for_each(|(n, expected)| assert_eq!(is_palindrome(*n), *expected));
}

#[test]
fn test_palindrome_sequence_length() {
    assert_eq!(generate_palindrome_sequence_length_map(48)[47].1, Some(1));
    assert_eq!(generate_palindrome_sequence_length_map(350)[349].1, Some(3));
    assert_eq!(generate_palindrome_sequence_length_map(197)[196].1, None);
}
