pub fn ans() -> u128 {
    let mut sum: u128 = 0;
    for i in 1..1_000_000 {
        if is_palindromic(i) && is_binary_palindromic(i) {
            sum += i as u128;
        }
    }
    sum
}

fn is_palindromic(n: u32) -> bool {
    let mut temp_n = n;
    let mut inverse_n: u32 = 0;
    while temp_n > 0 {
        inverse_n += temp_n % 10;
        temp_n /= 10;
        if temp_n > 0 {
            inverse_n *= 10;
        }
    }
    n == inverse_n
}

fn is_binary_palindromic(n: u32) -> bool {
    let mut temp_n = n as u128;
    let mut inverse_n: u128 = 0;
    while temp_n > 0 {
        inverse_n += temp_n % 2;
        temp_n /= 2;
        if temp_n > 0 {
            inverse_n *= 10;
        }
    }
    format!("{:b}", n).parse::<u128>().unwrap() == inverse_n
}
