use super::{spiral_corner_prime_ratio::PrimeRatio, spiral_corners::SpiralCorners};

#[test]
fn test_spiral_corners() {
    let expected: [[usize; 4]; 3] = [[3, 5, 7, 9], [13, 17, 21, 25], [31, 37, 43, 49]];
    let mut spiral_struct = SpiralCorners::default();
    (0..3).for_each(|i| {
        assert_eq!(spiral_struct.next().unwrap(), expected[i]);
    });
}

#[test]
fn test_spiral_corners_side_length() {
    let cases = [1, 3, 5, 7, 9];

    let mut spiral_struct = SpiralCorners::default();

    cases.iter().for_each(|expected| {
        assert_eq!(spiral_struct.side_length(), *expected);
        spiral_struct.next();
    })
}

#[test]
fn test_spiral_corner_prime_ratio() {
    let cases: [(u128, u128); 3] = [(3, 5), (5, 9), (8, 13)];

    let mut ratio = PrimeRatio::default();

    cases.iter().for_each(|expected| {
        let next = ratio.next().unwrap();
        assert_eq!((next.numerator, next.denominator), *expected)
    });
}
