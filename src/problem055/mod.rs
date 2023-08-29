pub fn ans() -> u128 {
    let max_n = 10_000;
    let palindrome_sequence_length_map = generate_palindrome_sequence_length_map(max_n);

    count_lychrel(palindrome_sequence_length_map)
}

fn count_lychrel(palindrome_sequence_length_map: Vec<(u128, Option<u128>)>) -> u128 {
    let lychrel_count =
        palindrome_sequence_length_map
            .iter()
            .fold(
                0u128,
                |sum, (_, palindrome_length)| match palindrome_length {
                    Some(_) => sum,
                    None => sum + 1,
                },
            );
    lychrel_count
}

fn generate_palindrome_sequence_length_map(max_n: u128) -> Vec<(u128, Option<u128>)> {
    let mut palindrome_sequence_length_map = (0u128..max_n)
        .map(|n| (n, Some(0)))
        .collect::<Vec<(u128, Option<u128>)>>();
    for n in (0u128..max_n).rev() {
        let rev_sum_n = reverse_sum(n);
        if is_palindrome(rev_sum_n) {
            palindrome_sequence_length_map[n as usize] = (n, Some(1))
        } else if rev_sum_n < max_n {
            match palindrome_sequence_length_map[rev_sum_n as usize] {
                (_, Some(k)) => {
                    if k + 1 >= 50 {
                        palindrome_sequence_length_map[n as usize] = (n, None)
                    } else {
                        palindrome_sequence_length_map[n as usize] = (n, Some(k + 1))
                    }
                }
                (_, None) => palindrome_sequence_length_map[n as usize] = (n, None),
            }
        } else {
            let mut k = 1;
            let mut rev_sum_k_n = rev_sum_n;
            while k < 50 && !is_palindrome(rev_sum_k_n) {
                k += 1;
                rev_sum_k_n = reverse_sum(rev_sum_k_n)
            }
            if is_palindrome(rev_sum_k_n) {
                palindrome_sequence_length_map[n as usize] = (n, Some(k))
            } else {
                palindrome_sequence_length_map[n as usize] = (n, None)
            }
        }
    }
    palindrome_sequence_length_map
}

fn is_palindrome(n: u128) -> bool {
    n == reverse(n)
}

fn reverse_sum(n: u128) -> u128 {
    n + reverse(n)
}

fn reverse(n: u128) -> u128 {
    let mut n = n;
    let mut rev_n = 0;
    while n >= 10 {
        rev_n += n % 10;
        rev_n *= 10;
        n /= 10;
    }
    rev_n += n % 10;
    rev_n
}

#[cfg(test)]
mod tests;
