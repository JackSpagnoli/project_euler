#[derive(Debug)]
pub struct OrderNumberClassifier {
    // The count of numbers less than 10^order which are increasing, decreasing, constant digit, or bouncy, indexed by starting digit
    pub increasing: [u128; 10],
    pub decreasing: [u128; 10],
    pub constant: [u128; 10],
    pub bouncy: [u128; 10],
    order: usize,
}

impl OrderNumberClassifier {
    pub fn get_non_bouncy(&self) -> u128 {
        self.increasing
            .iter()
            .chain(self.decreasing.iter())
            .chain(self.constant.iter())
            .sum()
    }

    pub fn get_bouncy(&self) -> u128 {
        self.bouncy.iter().sum()
    }

    pub fn get_order(&self) -> usize {
        self.order
    }
}

impl Iterator for OrderNumberClassifier {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.order += 1;

        let mut next_increasing = [0; 10];
        let mut next_decreasing = [0; 10];
        let mut next_constant = [0; 10];
        let mut next_bouncy = [0; 10];

        next_increasing[0] = self.increasing.iter().sum();
        next_decreasing[0] = self.decreasing.iter().sum();
        next_constant[0] = self.constant.iter().sum();
        next_bouncy[0] = self.bouncy.iter().sum();
        for new_first_digit in 1..10 {
            for old_first_digit in 1..10 {
                match new_first_digit.cmp(&old_first_digit) {
                    std::cmp::Ordering::Less => {
                        next_increasing[new_first_digit] +=
                            self.increasing[old_first_digit] + self.constant[old_first_digit];
                        next_bouncy[new_first_digit] +=
                            self.decreasing[old_first_digit] + self.bouncy[old_first_digit];
                    }
                    std::cmp::Ordering::Equal => {
                        next_increasing[new_first_digit] += self.increasing[old_first_digit];
                        next_decreasing[new_first_digit] += self.decreasing[old_first_digit];
                        next_constant[new_first_digit] += self.constant[old_first_digit];
                        next_bouncy[new_first_digit] += self.bouncy[old_first_digit];
                    }
                    std::cmp::Ordering::Greater => {
                        next_decreasing[new_first_digit] +=
                            self.decreasing[old_first_digit] + self.constant[old_first_digit];
                        next_bouncy[new_first_digit] +=
                            self.increasing[old_first_digit] + self.bouncy[old_first_digit];
                    }
                }
            }
            next_decreasing[new_first_digit] += 1;
            next_bouncy[new_first_digit] +=
                self.constant[0] + self.increasing[0] + self.decreasing[0] + self.bouncy[0];
        }

        self.increasing = next_increasing;
        self.decreasing = next_decreasing;
        self.constant = next_constant;
        self.bouncy = next_bouncy;

        Some(())
    }
}

impl Default for OrderNumberClassifier {
    fn default() -> Self {
        Self {
            increasing: [0; 10],
            decreasing: [0; 10],
            constant: [0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            bouncy: [0; 10],
            order: 1,
        }
    }
}
