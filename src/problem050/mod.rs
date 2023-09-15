use crate::primes::Primes;

pub fn ans() -> u128 {
    let mut primes = Primes::default();
    let mut d: u128 = 0;
    let mut p: u128 = 0;
    let mut n: usize = 0;
    while primes.index(n) < 1_000_000 / (d + 1) {
        let mut sum = primes.index(n);
        let mut k: usize = 1;
        while sum < 1_000_000 {
            sum += primes.index(n + k);
            if primes.is_prime(sum) && k + 1 > d as usize {
                d = (k + 1) as u128;
                p = sum;
            }
            k += 1;
        }
        n += 1;
    }
    p
}
