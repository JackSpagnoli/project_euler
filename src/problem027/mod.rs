pub fn ans() -> i128 {
    let mut primes: Vec<u128> = vec![2,3,5,7,11];
    let (min_a, max_a, min_b, max_b) = (-999i128, 999i128, -1000i128, 1000i128);
    let mut max_consec = 0;
    let mut max_a_b = (0,0);

    let mut a = min_a;
    while a<= max_a{
        let mut b = min_b;
        while b <= max_b{
            let mut n = 0;
            while is_prime(&mut primes,poly(n,a,b)){ n+=1; }
            if n>max_consec {
                max_a_b = (a, b);
                max_consec = n;
            }
            b+=1;
        }
        a+=1;
    }

    max_a_b.0 * max_a_b.1

}
fn poly(n:u128, a:i128, b:i128) -> i128{
    (n*n) as i128 + (a*n as i128) + b
}
fn generate_primes(primes:&mut Vec<u128>, n:i128){
    if n < primes[primes.len() - 1] as i128 {
        return;
    }
    
    let mut k = primes[primes.len() - 1] + 2;
    while k < n as u128 {
        if is_prime(primes, k as i128) { primes.push(k); }
        k+=2;
    }
}
fn is_prime(primes:&mut Vec<u128>, n:i128) -> bool{
    if n<2 {
        return false;
    }

    if primes[primes.len()-1] * primes[primes.len()-1] < n as u128 {
        generate_primes(primes, (primes[primes.len() - 1] * primes[primes.len() - 1]) as i128);
    }

    let mut i=0;
    while primes[i] * primes[i] <= n as u128 {
        if n as u128 %primes[i] == 0 { return false; }
        i+=1;
    }
    true
}