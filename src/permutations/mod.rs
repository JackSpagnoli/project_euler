use num_integer::Integer;

pub struct Permutations {
    elements: Vec<usize>,
    num_elements: usize,
    stack_state: Vec<usize>,
    stack_pointer: usize,
    first_iter: bool,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iter {
            self.first_iter = false;
            return Some(self.elements.clone());
        }
        loop {
            if self.stack_pointer >= self.num_elements {
                return None;
            } else if self.stack_state[self.stack_pointer] >= self.stack_pointer {
                self.stack_state[self.stack_pointer] = 0;
                self.stack_pointer += 1;
            }else{
                if self.stack_pointer.is_even() {
                    self.elements.swap(0, self.stack_pointer)
                } else {
                    self.elements.swap(self.stack_state[self.stack_pointer], self.stack_pointer)
                }
                self.stack_state[self.stack_pointer] += 1;
                self.stack_pointer = 1;

                return Some(self.elements.clone());
            }
        }
    }
}

impl From<usize> for Permutations {
    fn from(value: usize) -> Self {
        let elements: Vec<usize> = (0..value).collect();
        let num_elements = elements.len();
        let stack_state = vec![0; value];
        let stack_pointer = 1;
        let first_iter = true;
        Self {
            elements,
            num_elements,
            stack_state,
            stack_pointer,
            first_iter,
        }
    }
}

#[cfg(test)]
mod test;
