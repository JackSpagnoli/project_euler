use crate::number_utils::digits::Digits;

pub fn is_increasing(n: u128) -> bool {
    Digits::from(n)
        .collect::<Vec<usize>>()
        .windows(2)
        .all(|digits| digits[0] >= digits[1])
}

pub fn is_decreasing(n: u128) -> bool {
    Digits::from(n)
        .collect::<Vec<usize>>()
        .windows(2)
        .all(|digits| digits[0] <= digits[1])
}

pub fn is_bouncy(n: u128) -> bool {
    !is_increasing(n) && !is_decreasing(n)
}
