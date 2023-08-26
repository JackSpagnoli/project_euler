pub fn ans() -> u128{
    return iterate_2();
}
//Iterates through all possible d_2d_3d_4 combinations. Passes all valid ones to iterate().
fn iterate_2()->u128{
    let mut sum:u128 = 0;
    let mut d2:u8=0;
    let mut d3:u8=1;
    let mut d4:u8=2;
    while d2!=9 || d3!=8 || d4!=6{
        let mut used_digits:[bool;10]=[false;10];
        used_digits[d2 as usize] = true;
        if used_digits[d3 as usize]==false {
            used_digits[d3 as usize]=true;
            if used_digits[d4 as usize]==false {
                used_digits[d4 as usize]=true;
                let mut digits:[u8;10]=[0;10];
                digits[1]=d2;
                digits[2]=d3;
                digits[3]=d4;
                sum += iterate([3,5,7,11,13,17], 0, used_digits, digits);
            }
        }
        d4+=2;
        if d4==10 {
            d4=0;
            d3+=1;
            if d3==10 {
                d3=0;
                d2+=1;
                d2%=10;
            }
        }
    }
    return sum;
}
fn iterate(iterators:[u8;6], index:usize, used_digits:[bool;10], digits:[u8;10])->u128{
    let mut sum:u128 = 0;
    let prior:u16 = (100 * digits[index+2] as u16) + (10 * digits[index+3] as u16);
    let mut digits = digits.clone();
    while digits[index+4]<10 {
        if !used_digits[digits[index+4] as usize] {
            if (prior+digits[index+4] as u16) % iterators[index] as u16 == 0 {
                let mut used_digits = used_digits.clone();
                used_digits[digits[index+4] as usize] = true;
                if index<iterators.len()-1 {
                    sum += iterate(iterators, index + 1, used_digits, digits);
                }else{
                    let mut last_digit=0;
                    while used_digits[last_digit] {
                        last_digit+=1;
                    }
                    digits[0] = last_digit as u8;
                    sum += digits[9] as u128
                    + (10 * digits[8] as u128)
                    + (100 * digits[7] as u128)
                    + (1000 * digits[6] as u128)
                    + (10000 * digits[5] as u128)
                    + (100000 * digits[4] as u128)
                    + (1000000 * digits[3] as u128)
                    + (10000000 * digits[2] as u128)
                    + (100000000 * digits[1] as u128)
                    + (1000000000 * digits[0] as u128);
                }
            }
        }
        digits[index+4]+=1;
    }
    return sum;
}