pub(crate) struct SpiralCorners {
    pub(crate) side_radius: usize,
    pub(crate) prev: usize,
}

impl SpiralCorners {
    pub fn side_length(&self) -> usize {
        (self.side_radius * 2) + 1
    }
}

impl Iterator for SpiralCorners {
    type Item = [usize; 4];

    fn next(&mut self) -> Option<Self::Item> {
        self.side_radius += 1;
        let next = [
            self.prev + (2 * self.side_radius),
            self.prev + (4 * self.side_radius),
            self.prev + (6 * self.side_radius),
            self.prev + (8 * self.side_radius),
        ];
        self.prev = next[3];
        Some(next)
    }
}

impl Default for SpiralCorners {
    fn default() -> Self {
        Self {
            side_radius: 0,
            prev: 1,
        }
    }
}
