use super::order_number_classifier::OrderNumberClassifier;

#[test]
fn test_order_number_classifier() {
    let mut onc = OrderNumberClassifier::default();

    onc.next();

    assert_eq!(onc.increasing, [0, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(onc.decreasing, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(onc.constant, [9, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(onc.bouncy, [0; 10]);

    onc.next();

    assert_eq!(onc.increasing, [36, 44, 35, 27, 20, 14, 9, 5, 2, 0]);
    assert_eq!(onc.decreasing, [45, 2, 5, 9, 14, 20, 27, 35, 44, 54]);
    assert_eq!(onc.constant, [18, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(onc.bouncy, [0, 53, 59, 63, 65, 65, 63, 59, 53, 45]);

    onc.next();

    assert_eq!(onc.increasing, [192, 164, 119, 83, 55, 34, 19, 9, 3, 0]);
    assert_eq!(onc.decreasing, [255, 3, 9, 19, 34, 55, 83, 119, 164, 219]);
    assert_eq!(onc.constant, [27, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(
        onc.bouncy,
        [525, 832, 871, 897, 910, 910, 897, 871, 832, 780]
    );

    onc.next();

    assert_eq!(onc.increasing, [678, 494, 329, 209, 125, 69, 34, 14, 4, 0]);
    assert_eq!(
        onc.decreasing,
        [960, 4, 14, 34, 69, 125, 209, 329, 494, 714]
    );
    assert_eq!(onc.constant, [36, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(
        onc.bouncy,
        [8325, 9501, 9656, 9756, 9805, 9805, 9756, 9656, 9501, 9285]
    );

    onc.next();

    assert_eq!(
        onc.increasing,
        [1956, 1286, 791, 461, 251, 125, 55, 20, 5, 0]
    );
    assert_eq!(
        onc.decreasing,
        [2952, 5, 20, 55, 125, 251, 461, 791, 1286, 2001]
    );
    assert_eq!(onc.constant, [45, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(
        onc.bouncy,
        [95046, 98708, 99188, 99483, 99623, 99623, 99483, 99188, 98708, 97998]
    );
}

#[test]
fn test_non_bouncy_count() {
    let mut onc = OrderNumberClassifier::default();

    while onc.get_order() < 3 {
        onc.next();
    }
    assert_eq!(onc.get_non_bouncy() + onc.get_bouncy(), 999);
    assert_eq!(onc.get_non_bouncy(), 474);

    while onc.get_order() < 6 {
        onc.next();
    }
    assert_eq!(onc.get_non_bouncy() + onc.get_bouncy(), 999999);
    assert_eq!(onc.get_non_bouncy(), 12951);

    while onc.get_order() < 10 {
        onc.next();
    }
    assert_eq!(onc.get_non_bouncy() + onc.get_bouncy(), 9999999999);
    assert_eq!(onc.get_non_bouncy(), 277032);
}

use crate::problem112::utils::{is_decreasing, is_increasing};
#[test]
fn number_classification() {
    let mut increasing = [0; 10];
    let mut decreasing = [0; 10];
    let mut constant = [0; 10];
    let mut bouncy = [0; 10];
    let N = 1_000_000;
    for n in 1..N {
        let first_digit = n as usize / 100_000;
        match (is_increasing(n), is_decreasing(n)) {
            (false, false) => {
                bouncy[first_digit] += 1;
            }
            (false, true) => {
                decreasing[first_digit] += 1;
            }
            (true, false) => {
                increasing[first_digit] += 1;
            }
            (true, true) => {
                constant[first_digit] += 1;
            }
        }
    }
    println!("i:{increasing:?}");
    println!("d:{decreasing:?}");
    println!("c:{constant:?}");
    println!("b:{bouncy:?}");
    println!(
        "N-b:{}",
        N - bouncy.iter().copied().reduce(|a, k| a + k).unwrap() - 1
    );
}
