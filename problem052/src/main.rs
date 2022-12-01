use number_utils::get_digits;

fn main() {
    let mut x: u128 = 1;
    loop {
        let x_digits = get_digits(x);

        let mut i = 2;
        let mut invalid = false;
        while i <= 6 && !invalid {
            let multiple_digits = get_digits(i * x);
            if x_digits != multiple_digits {
                invalid = true;
            }
            i+=1;
        }

        if !invalid {
            println!("{x}");
            return;
        }

        x += 1;
    }
}