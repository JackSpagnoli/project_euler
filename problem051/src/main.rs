use primes::{ is_prime };
use std::env;

/**
ARGUMENTS:
NO ARGUMENTS: SEARCH FOR      6 PRIMES,      2 <= digits <= 7
 1 ARGUMENT : SEARCH FOR ARG[1] PRIMES,      2 <= digits <= 7
 2 ARGUMENTS: SEARCH FOR ARG[1] PRIMES,      2 <= digits <= ARG[2]
 3 ARGUMENTS: SEARCH FOR ARG[1] PRIMES, ARG[2] <= digits <= ARG[3]
 **/
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut digits: u32 = 1;
    let max_digits;
    let desired_prime_count;
    if args.len() == 2 {
        desired_prime_count = args[1].parse::<u8>().unwrap();
        max_digits = 7;
    } else if args.len() == 3 {
        desired_prime_count = args[1].parse::<u8>().unwrap();
        max_digits = args[2].parse::<u32>().unwrap();
    } else if args.len() == 4 {
        desired_prime_count = args[1].parse::<u8>().unwrap();
        max_digits = args[2].parse::<u32>().unwrap();
        digits = args[3].parse::<u32>().unwrap() - 1;
    } else {
        desired_prime_count = 6u8;
        max_digits = 7;
    }

    let mut primes: Vec<u64> = vec![2, 3, 5, 7];

    loop {
        if digits == max_digits {
            return;
        }
        digits += 1;
        for coding in 1..(2u64).pow(digits) {
            for static_digits in 0..(10u64).pow(
                format!("{coding:0>width$b}", width = digits as usize)
                    .chars()
                    .fold(
                        0,
                        |n: u32, digit: char|
                            n + (1 - digit.to_digit(2).expect("Error parsing: {digit}"))
                    )
            ) {
                let mut number_of_primes: u8 = 0;
                let mut print_buffer = vec!["Solution found:".to_string()];
                for replacement_digit in 0u64..10u64 {
                    let mut static_digits_copy = static_digits;
                    let num: u64 = format!("{coding:0>width$b}", width = digits as usize)
                        .chars()
                        .rev()
                        .fold(0u64, |mut n, digit| -> u64 {
                            n *= 10;
                            if digit.to_digit(2).expect("Error parsing: {digit}") == 1 {
                                n += replacement_digit;
                            } else {
                                n += static_digits_copy % 10;
                                static_digits_copy /= 10;
                            }
                            return n;
                        });
                    if is_prime(&mut primes, num) && num > (10u64).pow(digits - 1) {
                        number_of_primes += 1;
                        print_buffer.push(
                            format!(
                                "{coding:0>width$b} | {replacement_digit} | {static_digits} | {num}",
                                width = digits as usize
                            )
                        );
                    }
                }
                if number_of_primes == desired_prime_count {
                    for string in print_buffer {
                        println!("{string}");
                    }
                    return;
                }
            }
        }
    }
}