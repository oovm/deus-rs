use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

pub struct NQueens {}

pub struct NQueensBoard {
    rank: usize,
}

impl NQueensBoard {
    pub fn new(min: usize, max: usize) -> Self {
        Self { rank: 0 }
    }
}
