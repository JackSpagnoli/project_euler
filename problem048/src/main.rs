fn main() {
    let mut sum: u128 = 0;
    for i in 1..1001 as u128 {
        let mut product: u128 = 1;
        for _ in 0..i {
            product *= i;
            product %= 10_000_000_000;
        }
        sum += product;
        sum %= 10_000_000_000;
    }
    println!("Sum: {}", sum);
}
