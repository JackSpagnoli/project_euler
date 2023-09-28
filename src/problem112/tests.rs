use super::{
    find_bouncy_number_percentage,
    utils::{is_bouncy, is_decreasing, is_increasing},
};

#[test]
fn test_is_increasing() {
    let cases = [(134468, true), (66420, false), (155349, false)];

    cases
        .iter()
        .for_each(|(n, expected)| assert_eq!(is_increasing(*n), *expected));
}

#[test]
fn test_is_decreasing() {
    let cases = [(134468, false), (66420, true), (155349, false)];

    cases
        .iter()
        .for_each(|(n, expected)| assert_eq!(is_decreasing(*n), *expected));
}

#[test]
fn test_is_bouncy() {
    let cases = [(134468, false), (66420, false), (155349, true)];

    cases
        .iter()
        .for_each(|(n, expected)| assert_eq!(is_bouncy(*n), *expected));
}

#[test]
fn test_find_bouncy_number_percentage() {
    let cases = [(50, 538), (90, 21780)];

    cases
        .iter()
        .for_each(|(n, expected)| assert_eq!(find_bouncy_number_percentage(*n), *expected));
}
