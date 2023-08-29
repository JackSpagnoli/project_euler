pub fn ans() -> u128 {
    let mut primes: Vec<u128> = vec![2, 3, 5, 7, 11];
    generate_primes(&mut primes, 35_000);
    let mut max: u128 = 0;
    let mut n: u128 = 999_999_999;
    while n >= 9 {
        if n < max {
            n = 2;
        }
        if is_pandigital(n) && is_prime(&primes, n) && n > max {
            max = n;
        }
        n -= 2;
    }
    max
}

fn is_prime(primes: &[u128], n: u128) -> bool {
    let mut i: usize = 0;
    while primes[i] * primes[i] <= n {
        if n % primes[i] == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn generate_primes(primes: &mut Vec<u128>, limit: u128) {
    let mut n = primes[primes.len() - 1];
    while n <= limit {
        if is_prime(primes, n) {
            primes.push(n);
        }
        n += 2;
    }
}

fn is_pandigital(n: u128) -> bool {
    let mut number_of_digits: usize = 0;
    let mut temp: u128 = 1;
    while temp < n {
        temp *= 10;
        number_of_digits += 1;
    }
    let mut digits: Vec<bool> = vec![false];
    while number_of_digits > 1 {
        digits.push(false);
        number_of_digits -= 1;
    }
    let mut temp_n = n;
    while temp_n > 0 {
        let digit = temp_n % 10;
        if digit > digits.len() as u128 {
            return false;
        }
        if digit == 0 || digits[digit as usize - 1] {
            return false;
        }
        digits[digit as usize - 1] = true;
        temp_n /= 10;
    }
    true
}
