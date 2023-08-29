use std::cmp::max;

pub fn ans() -> u128 {
    max_digital_sum()
}

fn max_digital_sum() -> u128 {
    (2..100)
        .filter(is_root_coprime)
        .map(max_digital_sum_for_a)
        .max()
        .unwrap()
}

fn max_digital_sum_for_a(a: usize) -> u128 {
    let starting_digits: [usize; 200] = [vec![1usize; 1], vec![0usize; 199]]
        .concat()
        .try_into()
        .unwrap();
    (0..100)
        .fold((0u128, starting_digits), |(max_sum, digits), _| {
            let digits: Vec<usize> = digits
                .iter()
                .map(|digit| digit * a)
                .scan(0usize, |carry, digit| {
                    let digit = digit + *carry;
                    *carry = digit / 10;
                    Some(digit % 10)
                })
                .collect();

            let digit_sum: u128 = digits.iter().map(|digit| *digit as u128).sum();

            (max(digit_sum, max_sum), digits.try_into().unwrap())
        })
        .0
}

fn is_root_coprime(a: &usize) -> bool {
    let prime_factor_exponents = get_prime_factor_exponents(a)
        .iter()
        .filter(|exp| exp != &&0)
        .copied()
        .collect::<Vec<usize>>();
    if prime_factor_exponents.len() == 1 {
        return prime_factor_exponents[0] == 1;
    }
    are_coprime(prime_factor_exponents)
}

fn get_prime_factor_exponents(n: &usize) -> [usize; 25] {
    if n == &0 || n == &1 {
        return [0; 25];
    }
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    primes.map(|prime| {
        let mut exp: usize = 0;
        let mut prime_to_exp = prime;
        while n % prime_to_exp == 0 {
            exp += 1;
            prime_to_exp *= prime;
        }
        exp
    })
}

fn are_coprime(nums: Vec<usize>) -> bool {
    if nums.len() <= 1 {
        return true;
    }
    let exponents = nums
        .iter()
        .map(get_prime_factor_exponents)
        .map(|factors| factors.to_vec());

    const EMPTY_VECTOR: Vec<usize> = vec![];
    let restructure: [Vec<usize>; 25] =
        exponents.fold([EMPTY_VECTOR; 25], |mut restructure, exponents| {
            exponents
                .iter()
                .enumerate()
                .for_each(|(index, exponent)| restructure[index].push(*exponent));
            restructure
        });

    return restructure.iter().all(|factor_exponents| {
        factor_exponents.iter().min().expect(&format!(
            "nums:{nums:?}, factor_exponents:{factor_exponents:?}"
        )) == &0
    });
}

#[cfg(test)]
mod tests;
