pub fn ans() -> u128 {
    let max_a_b: u8 = 100;
    let sqrt_max_a_b: u8 = 10;

    let mut combinations: u128 = 0;

    let mut checked_a: [bool; 100 - 1] = [false; 100 - 1];

    loop {
        let mut a: u8 = 2;
        while checked_a[a as usize - 2] {
            a += 1;
            if a > max_a_b { break; }
        }
        if a > max_a_b { break; }

        combinations += max_a_b as u128 - 1;
        checked_a[a as usize - 2] = true;

        if a <= sqrt_max_a_b {
            let mut exponent_a: u16 = a as u16 * a as u16;
            let mut exponent: u8 = 2;
            while exponent_a <= max_a_b as u16 {

                for b in 2..max_a_b + 1 {
                    //println!("Checking b={}", b);
                    let mut valid: bool = true;
                    for prior in 1..exponent {
                        //println!("Checking prior exponent {}", prior);
                        if (exponent as u16 * b as u16) % prior as u16 == 0 && (exponent as u16 * b as u16) / prior as u16 <= max_a_b as u16 {
                            //println!("Found compatible prior exponent {}", prior);
                            valid = false;
                            break;
                        }
                    }
                    if valid { combinations += 1; }
                }

                checked_a[exponent_a as usize - 2] = true;
                exponent_a *= a as u16;
                exponent += 1;
            }
        }
    }
    return combinations;
}