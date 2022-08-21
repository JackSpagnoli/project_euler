fn main() {
    println!("Sum = {}", iterator(&mut [0u8;10],0));
}

fn digit_factorial(digits: [u8;10]) -> u128 {
    let mut factorial:u128 = 0;
    factorial+= digits[0] as u128 + digits[1] as u128;
    factorial+= 2u128*digits[2] as u128;
    factorial+= 62u128*digits[3] as u128;
    factorial+= 242u128*digits[4] as u128;
    factorial+= 1202u128*digits[5] as u128;
    factorial+= 7202u128*digits[6] as u128;
    factorial+= 2u128*digits[7] as u128;
    factorial+= 403202u128*digits[8] as u128;
    factorial+= 3628802u128*digits[9] as u128;
    return factorial;
}

fn iterator(digits: &mut [u8;10], index:usize) -> u128{
    let mut sum:u128=0;
    for i in 0..10 as u8{
        if index==2 {
            println!("Currently considering {}{}{}...", digits[0], digits[1], i);
        }
        digits[index] = i;
        //if smallest_possible_number(digits) <= digit_factorial(*digits) && index>0 {
            if index == 9 {
                let factorial = digit_factorial(*digits);
                if is_equal(&get_digits(factorial), digits) {
                    sum += factorial;
                }
            } else {
                sum += iterator(digits, index + 1);
            }
        //}
    }
    return sum;
}

fn get_digits(n:u128) -> [u8;10]{
    let mut temp_n:u128 = n.clone();
    let mut digits:[u8;10] = [0;10];
    while temp_n>0 {
        digits[(temp_n%10) as usize]+=1;
        temp_n/=10;
    }
    return digits;
}
fn is_equal(factorial_digits:&[u8;10], digits:&[u8;10])->bool{
    if factorial_digits[0] > digits[0] { return false; }
    for i in 1..10 {
        if factorial_digits[i] != digits[i] { return false; }
    }
    return true;
}
fn smallest_possible_number(digits:&[u8;10])->u128{
    let mut number:u128 = 0;
    for digit in (0..10).rev() {
        for _ in 0..digits[digit]+1 {
            number*=10;
            number+=digit as u128;
        }
    }
    return number;
}