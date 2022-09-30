use num_integer::Roots;
fn main() {
    let mut n:u128 = 286;
    while !tri_pent_hex(n){
        n+=1;
    }
    println!("{}", (n*(n+1))/2);
}
fn tri_pent_hex(n:u128)->bool{
    let sqrt:u128 = (1+(12*n*(n+1))).sqrt();
    if sqrt * sqrt != (1+(12*n*(n+1))) { return false; }
    let sqrt:u128 = (1+(4*n*(n+1))).sqrt();
    if sqrt * sqrt != (1+(4*n*(n+1))) { return false; }
    if (1+((1+(12*n*(n+1))).sqrt())) % 6 != 0 { return false; }
    if (1+(1+(4*n*(n+1))).sqrt()) % 4 != 0 { return false; }
    return true;
}
