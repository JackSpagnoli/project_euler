use crate::primes::{generate_primes, is_prime};

pub fn ans() -> u128 {
    let mut primes:Vec<u64> = vec![2,3,5,7];
    generate_primes(&mut primes, 1_000_000);
    let mut d:u64 = 0;
    let mut p:u64 = 0;
    let mut n:usize = 0;
    while primes[n] < 1_000_000 / (d+1){
        let mut sum = primes[n];
        let mut k:usize = 1;
        while sum < 1_000_000{
            sum+=primes[n+k];
            if is_prime(&mut primes, sum) && k+1>d as usize{
                d = (k+1) as u64;
                p = sum;
            }
            k+=1;
        }
        n+=1;
    }
    p as u128
}