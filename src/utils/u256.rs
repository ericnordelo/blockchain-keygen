use std::fmt;

pub struct U256 {
    slices: [u128; 2],
}

impl U256 {
    pub fn new(slices: [u128; 2]) -> Self {
        Self { slices }
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032x}{:032x}", self.slices[0], self.slices[1])
    }
}
