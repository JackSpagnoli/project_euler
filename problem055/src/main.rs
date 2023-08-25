fn main() {
    let max_n = 10_000;
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

    assert_eq!(palindrome_sequence_length_map[47].1, Some(1));
    assert_eq!(palindrome_sequence_length_map[349].1, Some(3));
    assert_eq!(palindrome_sequence_length_map[196].1, None);

    let lychrel_count =
        palindrome_sequence_length_map
            .iter()
            .fold(
                0usize,
                |sum, (_, palindrome_length)| match palindrome_length {
                    Some(_) => sum,
                    None => sum + 1,
                },
            );

    println!("{lychrel_count}");
}

fn is_palindrome(n: u128) -> bool {
    return n == reverse(n);
}

fn reverse_sum(n: u128) -> u128 {
    return n + reverse(n);
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
    return rev_n;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_reverse() {
        let cases: [(u128, u128); 5] = [
            (1, 1),
            (12, 21),
            (123, 321),
            (3478923432, 2343298743),
            (1067869, 9687601),
        ];

        cases
            .iter()
            .for_each(|(input, expected)| assert_eq!(reverse(*input), *expected))
    }

    #[test]
    fn test_reverse_sum() {
        let cases: [(u128, u128); 4] = [(1, 2), (12, 33), (123, 444), (3478923432, 5822222175)];

        cases
            .iter()
            .for_each(|(input, expected)| assert_eq!(reverse_sum(*input), *expected))
    }

    #[test]
    fn test_palindrome() {
        let cases = [
            (1, true),
            (11, true),
            (12, false),
            (123, false),
            (121, true),
            (34543, true),
        ];

        cases
            .iter()
            .for_each(|(n, expected)| assert_eq!(is_palindrome(*n), *expected));
    }
}
