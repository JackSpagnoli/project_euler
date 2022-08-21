fn main() {
    let mut primes: Vec<u128> = vec!(2, 3, 5, 7, 11);

    let mut truncatable_primes: Vec<u128> = vec![];
    let mut i = primes.len() - 1;
    while truncatable_primes.len() < 11 {
        let prime = primes[i].clone();
        if is_right_truncatable(&mut primes, prime) {
            if is_left_truncatable(&mut primes, prime) {
                truncatable_primes.push(prime);
                println!("Found truncatable prime {}. Have now found {}", prime, truncatable_primes.len());
            }
        }
        i += 1;
        if i == primes.len() { generate_primes(&mut primes, prime * 2); }
    }
    let mut sum: u128 = 0;
    for truncatable_prime in truncatable_primes {
        print!("{}, ", truncatable_prime);
        sum += truncatable_prime;
    }
    println!();
    println!("Sum: {}", sum);
}

fn is_right_truncatable(primes: &mut Vec<u128>, n: u128) -> bool {
    if !primes.binary_search(&n).is_ok() { return false; }
    if n < 10 { return true; }
    return is_right_truncatable(primes, n / 10);
}

fn is_left_truncatable(primes: &mut Vec<u128>, n: u128) -> bool {
    if !primes.binary_search(&n).is_ok() { return false; }
    if n < 10 { return true; }

    let mut modulo: u128 = 1;
    let mut temp_n = n;
    while temp_n > 0 {
        modulo *= 10;
        temp_n /= 10;
    }
    modulo /= 10;
    return is_left_truncatable(primes, n % modulo);
}

fn generate_primes(primes: &mut Vec<u128>, n: u128) {
    println!("Generating primes up to {}", n);
    if n < primes[primes.len() - 1] {
        return;
    }

    let mut k = primes[primes.len() - 1].clone() + 2;
    while k < n as u128 {
        if is_prime(primes, k) { primes.push(k); }
        k += 2;
    }
}

fn is_prime(primes: &mut Vec<u128>, n: u128) -> bool {
    let mut i = 0;
    while primes[i] * primes[i] <= n {
        if n % primes[i] == 0 { return false; }
        i += 1;
    }
    return true;
}