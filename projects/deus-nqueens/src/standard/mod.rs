use num::Integer;
use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

pub struct NQueens {}

#[derive(Clone, Debug)]
pub struct NQueensBoard<const N: usize> {
    arrange: [usize; N],
}

impl NQueens {
    pub fn solve() {}
}

#[test]
fn test() {
    println!("{:?}", NQueensBoard::pre_solved(8))
}

impl<const N: usize> NQueensBoard<N> {
    pub fn is_solution(&self) -> bool {
        false
    }

    /// https://arxiv.org/abs/1805.07329
    fn pre_solved() -> Option<Self> {
        let mut arrange = [0; N];
        match n + 1 {
            // Lemma 1.6: n = 12k-4
            m if n + 4 % 12 == 0 => {
                for i in 0..n {
                    match i > n / 2 {
                        true if i.is_odd() => arrange[i] = (2 * i + 2) % m,
                        true if i.is_even() => arrange[i] = (2 * i - 2) % m,
                        _ => arrange[i] = (2 * i) % m,
                    }
                }
            }
            _ => {
                panic!()
            }
        }

        Some(Self { arrange })
    }
}
