pub fn generate_primes(primes: &mut Vec<u64>, n: u64) {
    if n < primes[primes.len() - 1] {
        return;
    }

    let mut k = primes[primes.len() - 1] + 2;
    while k < n {
        if is_prime(primes, k) { primes.push(k); }
        k += 2;
    }
}

pub fn is_prime(primes: &mut Vec<u64>, n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if primes[primes.len() - 1] * primes[primes.len() - 1] < n {
        generate_primes(primes, primes[primes.len() - 1] * primes[primes.len() - 1]);
    }

    let mut i: usize = 0;
    while primes[i] * primes[i] <= n {
        if n % primes[i] == 0 { return false; }
        i += 1;
    }
    true
}