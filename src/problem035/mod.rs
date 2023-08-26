pub fn ans() -> u128 {
    let mut primes: [u32; 78498] = primes_under_million();

    let mut circular_primes: u32 = 0;
    let mut i: usize = 0;
    while i < 78498 {
        if primes[i] != 0 {
            let mut circular_prime = true;
            let mut period: u8 = 1;
            let mut cycler = cycle(primes[i]);
            let mut indexes = vec![i];
            while cycler != primes[i] {
                let index = primes.binary_search(&cycler);
                if index.is_ok() {
                    period += 1;
                    indexes.push(index.unwrap());
                    cycler = cycle(cycler);
                } else {
                    circular_prime = false;
                    cycler = primes[i];
                }
            }
            if circular_prime {
                for index in indexes {
                    primes[index] = 0;
                }
                circular_primes += period as u32;
            }
        }
        i += 1;
    }
    return circular_primes as u128;
}

fn cycle(n: u32) -> u32 {
    if n < 10 { return n; }
    let last_digit: u8 = (n % 10) as u8;
    let remainder: u32 = n / 10;

    let mut i: u32 = 10;
    while remainder > i {
        i *= 10;
    }
    return ((i) * (last_digit as u32)) + remainder;
}

fn primes_under_million() -> [u32; 78498] {
    let mut primes: [u32; 78498] = [0; 78498];
    primes[0] = 2;
    primes[1] = 3;

    let mut n: u32 = 5;
    let mut i = 2;
    while i < 78498 {
        let mut j = 0;
        let mut prime: bool = true;
        while j < i && primes[j] * primes[j] <= n {
            if n % primes[j] == 0 {
                j = i;
                prime = false;
            }
            j += 1;
        }
        if prime {
            primes[i] = n;
            i += 1;
        }
        n += 2;
    }
    return primes;
}