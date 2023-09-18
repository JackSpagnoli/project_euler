use crate::number_utils::digit_dist::DigitDistribution;

pub fn ans() -> u128 {
    let mut x: u128 = 1;
    loop {
        let x_digits = DigitDistribution::from(x).get_distribution();

        let mut i = 2;
        let mut invalid = false;
        while i <= 6 && !invalid {
            let multiple_digits = DigitDistribution::from(i * x).get_distribution();
            if x_digits != multiple_digits {
                invalid = true;
            }
            i += 1;
        }

        if !invalid {
            return x;
        }

        x += 1;
    }
}
