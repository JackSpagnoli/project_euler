fn main() {
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
    println!("{}", p);
}
fn generate_primes(primes:&mut Vec<u64>, n:u64){
    if n < primes[primes.len() - 1] {
        return;
    }

    let mut k = primes[primes.len() - 1].clone() + 2;
    while k < n {
        if is_prime(primes, k) { primes.push(k); }
        k+=2;
    }
}
fn is_prime(primes:&mut Vec<u64>, n:u64) -> bool{
    if n<2 {
        return false;
    }

    if primes[primes.len()-1].clone() * primes[primes.len()-1].clone() < n {
        generate_primes(primes, primes[primes.len() - 1].clone() * primes[primes.len() - 1].clone());
    }

    let mut i:usize=0;
    while primes[i].clone() * primes[i].clone() <= n {
        if n %primes[i] == 0 { return false; }
        i+=1;
    }
    return true;
}