pub fn ans() -> u128 {
    let mut primes: Vec<u64> = vec![2, 3, 5, 7];
    generate_primes(&mut primes, 9_973);
    primes.reverse();
    primes.truncate(1061);
    primes.reverse();
    for n in &primes {
        for k in 2..((9973 - n) / 2) {
            if primes.contains(&(n + k)) {
                if primes.contains(&(n + k + k)) {
                    if permutation(n, &(n + k)) {
                        if permutation(n, &(n + k + k)) {
                            if permutation(&(n + k), &(n + k + k)) {
                                if *n != 1487 {
                                    let n_1: u128 = *n as u128;
                                    let n_2: u128 = n_1 + k as u128;
                                    let n_3: u128 = n_2 + k as u128;
                                    return (1_0000_0000 * n_1) + (1_0000 * n_2) + n_3;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0;
}

fn generate_primes(primes: &mut Vec<u64>, n: u64) {
    let mut temp_n = primes[primes.len() - 1] + 2;
    while primes[primes.len() - 1] < n {
        if primes
            .iter()
            .map(|x| temp_n % x == 0)
            .filter(|x| *x)
            .collect::<Vec<bool>>()
            .len()
            == 0
        {
            primes.push(temp_n);
        }
        temp_n += 2;
    }
}

fn permutation(a: &u64, b: &u64) -> bool {
    let digits_a: [u64; 4] = [a % 10, (a / 10) % 10, (a / 100) % 10, (a / 1000) % 10];

    if !digits_a.contains(&(b % 10)) {
        return false;
    }
    if !digits_a.contains(&((b / 10) % 10)) {
        return false;
    }
    if !digits_a.contains(&((b / 100) % 10)) {
        return false;
    }
    if !digits_a.contains(&((b / 1000) % 10)) {
        return false;
    }

    let digits_b: [u64; 4] = [b % 10, (b / 10) % 10, (b / 100) % 10, (b / 1000) % 10];

    if !digits_b.contains(&(a % 10)) {
        return false;
    }
    if !digits_b.contains(&((a / 10) % 10)) {
        return false;
    }
    if !digits_b.contains(&((a / 100) % 10)) {
        return false;
    }
    if !digits_b.contains(&((a / 1000) % 10)) {
        return false;
    }

    return true;
}
