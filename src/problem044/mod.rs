pub fn ans() -> u128{
    let mut total_valid_numbers: u128 = 0;
    let mut i: u128 = 1;
    let mut d = u128::MAX;

    //Iterates over all (i,j) pairs with j<i, checking if each pair is a valid combination. If so, d is updated and the count of valid pairs is increased.
    //All numbers are 2*P_n
    while total_valid_numbers < 1 {
        let twice_i_pentagonal = i * ((3 * i) - 1);
        for j in 1..i {
            let twice_j_pentagonal = j * ((3 * j) - 1);
            if valid_pair(twice_i_pentagonal, twice_j_pentagonal) {
                total_valid_numbers += 1;
                if twice_i_pentagonal - twice_j_pentagonal < d {
                    d = twice_i_pentagonal - twice_j_pentagonal;
                }
            }
        }
        i += 1;
    }
    d / 2
}

//Checks if the passed number is 2*P_n for some n.
fn is_pentagonal_number(twice_n: u128) -> bool {
    let mut n: u128 = 1;
    let mut pent: u128 = 2;
    while pent <= twice_n {
        if pent == twice_n { return true; }
        n += 1;
        pent = n * ((3 * n) - 1);
    }
    false
}

//Checks if the sum and difference of the two passed numbers are pentagonal
fn valid_pair(i_pent: u128, j_pent: u128) -> bool {
    if is_pentagonal_number(i_pent - j_pent) { is_pentagonal_number(i_pent + j_pent) } else { false }
}