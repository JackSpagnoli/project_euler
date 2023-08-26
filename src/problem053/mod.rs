pub fn ans() -> u128 {
    let n_range = 23..=100;

    let vals: u128 = n_range.map(evaluate_n).sum();

    return vals;
}
fn evaluate_n(n: u128) -> u128 {
    let r_max = n / 2;

    let range: Vec<u128> = (0..=r_max).collect();
    let res = range
        .binary_search_by(|r| {
            let ncr_r = ncr_greater_than_mill(n, *r);
            let ncr_r_minus_1 = ncr_greater_than_mill(n, *r - 1);

            if ncr_r && !ncr_r_minus_1 {
                return std::cmp::Ordering::Equal;
            } else if ncr_r {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        })
        .unwrap() as u128;

    return n - (2 * res) + 1;
}

fn ncr_greater_than_mill(n: u128, r: u128) -> bool {
    let mut num: f64 = 1.0;
    for i in (n - r + 1)..=(n) {
        num *= i as f64;
    }
    for i in 2..=r {
        num /= i as f64;

        if num < 1_000_000.0 {
            return false;
        }
    }
    return true;
}
