pub fn ans() -> u128 {
    let ranges: [(u64, u64); 3] = [(25u64, 33u64), (100u64, 329u64), (5000u64, 9876u64)];
    let mut max_pandigital: u64 = 0;
    for range in ranges {
        let mut m = range.1;
        while m >= range.0 {
            let (is_pandigital, pandigital_value) = is_pandigital(m);
            if is_pandigital && pandigital_value > max_pandigital {
                max_pandigital = pandigital_value;
            }
            m -= 1;
        }
    }
    max_pandigital as u128
}

fn is_pandigital(m: u64) -> (bool, u64) {
    let mut digits: [bool; 9] = [false; 9];
    let mut pandigital = m;

    let mut temp_m = m;
    while temp_m > 0 {
        let digit: usize = (temp_m % 10) as usize;
        if digit == 0 && temp_m != 0 {
            return (false, 0);
        }
        if digits[digit - 1] {
            return (false, 0);
        }
        digits[digit - 1] = true;
        temp_m /= 10;
    }
    let mut n: u64 = 2;
    while digits.contains(&false) {
        let product = m * n;
        let mut temp_product = product;
        while temp_product > 0 {
            let digit: usize = (temp_product % 10) as usize;
            if digit == 0 && temp_product != 0 {
                return (false, 0);
            }
            if digits[digit - 1] {
                return (false, 0);
            }
            digits[digit - 1] = true;
            temp_product /= 10;
            pandigital *= 10;
        }
        pandigital += product;
        n += 1;
    }

    (true, pandigital)
}
