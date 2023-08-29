pub fn ans() -> u128 {
    let mut primes: Vec<u64> = vec![2, 3, 5, 7];
    let mut n: u64 = 33;
    while can_be_written(&mut primes, n) && !is_prime(&primes, n) {
        n += 2;
    }
    n as u128
}

fn can_be_written(primes: &mut Vec<u64>, n: u64) -> bool {
    generate_primes(primes, n);
    if is_prime(primes, n) {
        return false;
    }
    let mut i: u64 = 1;
    while primes[i as usize] <= n {
        let mut j: u64 = 0;
        while primes[i as usize] + (2 * j * j) < n {
            j += 1;
        }
        if primes[i as usize] + (2 * j * j) == n {
            return true;
        }
        i += 1;
    }
    false
}

fn generate_primes(primes: &mut Vec<u64>, n: u64) {
    while primes[primes.len() - 1] < n {
        let mut i = primes[primes.len() - 1] + 2;
        while !is_prime(primes, i) {
            i += 2;
        }
        primes.push(i);
    }
}

fn is_prime(primes: &Vec<u64>, n: u64) -> bool {
    let mut i = 0;
    while i < primes.len() {
        if n % primes[i] == 0 {
            return false;
        }
        i += 1;
    }
    true
}
