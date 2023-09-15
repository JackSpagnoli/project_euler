pub struct Primes {
    primes: Vec<u128>,
    index: usize,
}

impl Primes {
    pub fn reset(self) -> Self {
        Primes { index: 0, ..self }
    }
    pub fn is_prime(&mut self, n: u128) -> bool {
        while self.has_enough_primes(n) {
            self.generate_next_prime();
        }

        self.primes
            .iter()
            .filter(|p| *p * *p <= n)
            .all(move |p| n % p != 0)
    }

    fn highest_prime(&self) -> &u128 {
        self.primes.last().unwrap()
    }

    fn has_enough_primes(&self, n: u128) -> bool {
        let highest_prime: &u128 = self.highest_prime();
        highest_prime * highest_prime <= n
    }

    fn generate_next_prime(&mut self) -> u128 {
        let mut next_value = self.primes.last().unwrap() + 2;
        while !self.is_prime(next_value) {
            next_value += 2;
        }

        self.primes.push(next_value);
        next_value
    }

    pub fn index(&mut self, index: usize) -> u128 {
        while index >= self.primes.len() {
            self.generate_next_prime();
        }
        self.primes[index]
    }
}

impl Iterator for Primes {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.primes.len() {
            self.index += 1;
            return Some(self.primes[self.index - 1]);
        }
        self.index += 1;
        Some(self.generate_next_prime())
    }
}

impl Default for Primes {
    fn default() -> Self {
        let primes: Vec<u128> = vec![2, 3];
        let index = 0;
        Self { primes, index }
    }
}

#[cfg(test)]
mod tests;
