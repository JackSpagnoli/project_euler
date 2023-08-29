pub fn get_digits(n: u128) -> [u64; 10] {
    let mut n_copy = n;
    let mut digits: [u64; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    if n_copy == 0 {
        digits[0] += 1;
    }
    while n_copy > 0 {
        digits[(n_copy % 10) as usize] += 1;
        n_copy /= 10;
    }
    digits
}