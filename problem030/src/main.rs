fn main() {
    let mut sum: u128 = 0;
    let mut n: u128 = 2;
    let max_n: u128 = 1000000;
    while n < max_n {
        if digit_pow_sum(n) == n {
            println!("Found n:{}", n);
            sum += n;
        }
        n += 1;
    }

    println!("Final sum of: {}", sum);
}

fn digit_pow_sum(n: u128) -> u128 {
    let mut temp_n: u128 = n;
    let mut sum: u128 = 0;
    while temp_n > 0 {
        let digit: u128 = temp_n % 10;
        sum += fifth_pow(digit);
        temp_n /= 10;
    }
    return sum;
}

fn fifth_pow(n: u128) -> u128 {
    return n * n * n * n * n;
}