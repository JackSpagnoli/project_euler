pub fn ans() -> u128 {
    let mut sum: u128 = 0;
    for i in 3..25_000_000_u128 {
        if digit_factorial(get_digits(i)) == i {
            sum += i;
        }
    }
    sum
}

fn digit_factorial(digits: [u8; 10]) -> u128 {
    let mut factorial: u128 = 0;
    factorial += digits[0] as u128 + digits[1] as u128;
    factorial += 2u128 * digits[2] as u128;
    factorial += 6u128 * digits[3] as u128;
    factorial += 24u128 * digits[4] as u128;
    factorial += 120u128 * digits[5] as u128;
    factorial += 720u128 * digits[6] as u128;
    factorial += 5040u128 * digits[7] as u128;
    factorial += 40320u128 * digits[8] as u128;
    factorial += 362880u128 * digits[9] as u128;
    factorial
}

fn get_digits(n: u128) -> [u8; 10] {
    let mut temp_n: u128 = n;
    let mut digits: [u8; 10] = [0; 10];
    while temp_n > 0 {
        digits[(temp_n % 10) as usize] += 1;
        temp_n /= 10;
    }
    digits
}