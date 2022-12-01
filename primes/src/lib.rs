pub fn generate_primes(primes: &mut Vec<u64>, n: u64) {
    if n < primes[primes.len() - 1] {
        return;
    }

    let mut k = primes[primes.len() - 1].clone() + 2;
    while k < n {
        if is_prime(primes, k) { primes.push(k); }
        k += 2;
    }
}

pub fn is_prime(primes: &mut Vec<u64>, n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if primes[primes.len() - 1].clone() * primes[primes.len() - 1].clone() < n {
        generate_primes(primes, primes[primes.len() - 1].clone() * primes[primes.len() - 1].clone());
    }

    let mut i: usize = 0;
    while primes[i].clone() * primes[i].clone() <= n {
        if n % primes[i] == 0 { return false; }
        i += 1;
    }
    return true;
}