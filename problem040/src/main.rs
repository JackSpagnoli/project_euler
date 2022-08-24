fn main() {
    let mut n: u64 = 1;
    let mut digit: u64 = 0;
    let mut product: u64 = 1;
    while digit <= 1_000_000 {
        let mut temp_n = n;
        digit += digits(n);
        while temp_n > 0 {
            if power_of_ten(digit) {
                println!("{}, {}, {}", n, digit, temp_n % 10);
                product *= temp_n % 10;
            }
            digit -= 1;
            temp_n /= 10;
        }
        digit += digits(n);
        n += 1;
    }
    println!("Product: {}", product);
}

fn digits(n: u64) -> u64 {
    let mut temp_n = n;
    let mut digits: u64 = 0;
    while temp_n > 0 {
        digits += 1;
        temp_n /= 10;
    }
    return digits;
}

fn power_of_ten(n: u64) -> bool {
    let mut temp_n = n;
    while temp_n > 1 {
        if temp_n % 10 > 0 { return false; }
        temp_n /= 10;
    }
    return true;
}