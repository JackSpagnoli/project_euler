pub fn ans() -> u128 {
    let mut primes: Vec<u64> = vec![2, 3, 5, 7];
    let mut consecutives: u8 = 0;
    let mut n: u64 = 1;
    while consecutives < 4 {
        let mut i: usize = 0;
        let mut temp_n = n;
        let mut distinct_prime_factors: u8 = 0;
        while i < primes.len() && temp_n > 1 {
            let pre_n = temp_n;
            while temp_n % primes[i] == 0 {
                temp_n /= primes[i];
            }
            if temp_n < pre_n {
                distinct_prime_factors += 1;
            }
            i += 1;
        }
        if distinct_prime_factors == 4 {
            consecutives += 1;
        } else {
            consecutives = 0;
        }
        n += 1;
        generate_primes(&mut primes, n);
        while primes.contains(&n) && consecutives < 4 {
            consecutives = 0;
            n += 1;
            generate_primes(&mut primes, n);
        }
    }
    n as u128 - 4
}

fn generate_primes(primes: &mut Vec<u64>, n: u64) {
    let mut temp_n = primes[primes.len() - 1] + 2;
    while primes[primes.len() - 1] < n {
        if primes
            .iter()
            .map(|x| temp_n % x == 0)
            .filter(|x| *x)
            .collect::<Vec<bool>>()
            .is_empty()
        {
            primes.push(temp_n);
        }
        temp_n += 2;
    }
}
