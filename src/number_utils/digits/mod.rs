#[cfg(test)]
mod test;

pub struct Digits {
    n: Option<u128>,
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.n {
            let digit = n % 10;
            let temp_n = n / 10;
            if temp_n == 0 {
                self.n = None
            } else {
                self.n = Some(temp_n)
            }
            Some(digit as usize)
        } else {
            None
        }
    }
}

impl From<u128> for Digits {
    fn from(value: u128) -> Self {
        Self { n: Some(value) }
    }
}
