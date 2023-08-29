pub fn ans() -> u128 {
    let mut primes: Vec<u128> = vec![2, 3, 5, 7, 11];

    let mut truncatable_primes: Vec<u128> = vec![];
    let mut i = primes.len() - 1;
    while truncatable_primes.len() < 11 {
        let prime = primes[i];
        if is_right_truncatable(&mut primes, prime) && is_left_truncatable(&mut primes, prime) {
            truncatable_primes.push(prime);
        }
        i += 1;
        if i == primes.len() {
            generate_primes(&mut primes, prime * 2);
        }
    }
    let mut sum: u128 = 0;
    for truncatable_prime in truncatable_primes {
        sum += truncatable_prime;
    }
    sum
}

fn is_right_truncatable(primes: &mut Vec<u128>, n: u128) -> bool {
    if primes.binary_search(&n).is_err() {
        return false;
    }
    if n < 10 {
        return true;
    }
    is_right_truncatable(primes, n / 10)
}

fn is_left_truncatable(primes: &mut Vec<u128>, n: u128) -> bool {
    if primes.binary_search(&n).is_err() {
        return false;
    }
    if n < 10 {
        return true;
    }

    let mut modulo: u128 = 1;
    let mut temp_n = n;
    while temp_n > 0 {
        modulo *= 10;
        temp_n /= 10;
    }
    modulo /= 10;
    is_left_truncatable(primes, n % modulo)
}

fn generate_primes(primes: &mut Vec<u128>, n: u128) {
    if n < primes[primes.len() - 1] {
        return;
    }

    let mut k = primes[primes.len() - 1] + 2;
    while k < n {
        if is_prime(primes, k) {
            primes.push(k);
        }
        k += 2;
    }
}

fn is_prime(primes: &[u128], n: u128) -> bool {
    let mut i = 0;
    while primes[i] * primes[i] <= n {
        if n % primes[i] == 0 {
            return false;
        }
        i += 1;
    }
    true
}
