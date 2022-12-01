use std::env;

/**
ARGUMENTS:
NO ARGUMENTS: 2 <= digits <= 7
 1 ARGUMENT : 2 <= digits <= ARG[1]
 2 ARGUMENTS: ARG[1] <= digits <= ARG[2]
 **/
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut digits: u32 = 2;
    let max_digits;
    if args.len() == 2 {
        max_digits = args[1].parse::<u32>().unwrap();
    } else if args.len() == 3 {
        digits = args[1].parse::<u32>().unwrap();
        max_digits = args[2].parse::<u32>().unwrap();
    } else {
        max_digits = 7;
    }

    // let mut primes: Vec<u64> = vec![2, 3, 5, 7];


    loop {
        if digits > max_digits { return; }
        digits += 1;
        for coding in 1..2u64.pow(digits) {
            for static_digits in 0..10u64.pow(
                format!("{coding:b}")
                    .chars()
                    .fold(0,
                          |n: u32, digit: char|
                              n + (1 - digit.to_digit(2).unwrap()),
                    )
            ) {
                // let mut number_of_primes: u8 = 0;
                for replacement_digit in 0u64..10u64 {
                    let mut static_digits_copy = static_digits;
                    let num: u64 = format!("{coding:b}")
                        .chars().rev()
                        .fold(0u64,
                              |mut n, digit| -> u64 {
                                  n *= 10;
                                  if digit.to_digit(2).unwrap() == 1 { n += replacement_digit } else {
                                      n += static_digits_copy % 10;
                                      static_digits_copy /= 10;
                                  }
                                  return n;
                              });
                    println!("{num}")
                    // if is_prime(&mut primes, num) {number_of_primes==1;}
                }
                // if number_of_primes == 6 {
                //     println!("")
                // }
            }
        }
    }
}
