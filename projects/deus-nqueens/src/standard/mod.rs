use num::Integer;
use std::fmt::{Debug, Display, Formatter};

mod display;

pub struct NQueens {}

#[derive(Clone)]
pub struct NQueensBoard {
    arrange: Vec<usize>,
}

impl NQueens {
    pub fn solve(rank: usize) -> NQueensBoard {
        unsafe {
            match NQueensBoard::pre_solved(rank) {
                Some(s) => return s,
                None => panic!(),
            }
        }
    }
}

#[test]
fn test() {
    unsafe {
        for i in 0..=24 {
            println!("{}: {:?}", i, NQueensBoard::pre_solved(i));
        }
    }
}

impl NQueensBoard {
    pub fn is_solution(&self) -> bool {
        false
    }

    /// https://arxiv.org/abs/1805.07329
    unsafe fn pre_solved(n: usize) -> Option<Self> {
        let mut arrange = vec![0; n];
        match n + 1 {
            1 => return None,
            2 => *arrange.get_unchecked_mut(0) = 1,
            3 | 4 => return None,
            // Lemma 1.7: n = 6k, 6k+4
            m if (n - 4) % 6 == 0 || n % 6 == 0 => {
                for i in 1..=n {
                    *arrange.get_unchecked_mut(i - 1) = (2 * i) % m
                }
            }
            // Lemma 1.8: n = 6k + 1, 6k + 5
            m if (n - 1) % 6 == 0 || (n - 5) % 6 == 0 => {
                for i in 1..=n {
                    match n / 2 {
                        k if i > k => *arrange.get_unchecked_mut(i - 1) = (2 * i + 1) % m,
                        _ => *arrange.get_unchecked_mut(i - 1) = (2 * i) % m,
                    }
                }
            }
            // Lemma 1.10: n = 6k + 3
            m if (n - 3) % 6 == 0 => {
                for i in 1..=n {
                    match (n - 1) / 2 {
                        k if i < k => *arrange.get_unchecked_mut(i - 1) = (2 * i + 2) % m,
                        k if i > k => *arrange.get_unchecked_mut(i - 1) = (2 * i + 5) % m,
                        _ => *arrange.get_unchecked_mut(i - 1) = (2 * i + 4) % m,
                    }
                }
            }
            // Lemma 1.6: n = 12k-4
            m if (n + 4) % 12 == 0 => {
                for i in 1..=n {
                    match n / 2 {
                        k if n > k && i.is_odd() => *arrange.get_unchecked_mut(i - 1) = (2 * i + 2) % m,
                        k if n > k && i.is_even() => *arrange.get_unchecked_mut(i - 1) = (2 * i - 2) % m,
                        _ => *arrange.get_unchecked_mut(i - 1) = (2 * i) % m,
                    }
                }
            }
            // Lemma 1.9: n = 12k + 2
            m if (n - 2) % 12 == 0 => {
                for i in 1..=n {
                    match n / 2 {
                        _ if i == n => *arrange.get_unchecked_mut(i - 1) = (2 * i + 4) % m,
                        k if i < k && i.is_odd() => *arrange.get_unchecked_mut(i - 1) = (2 * i + 4) % m,
                        k if i < k && i.is_even() => *arrange.get_unchecked_mut(i - 1) = (2 * i) % m,
                        _ => *arrange.get_unchecked_mut(i - 1) = (2 * i + 2) % m,
                    }
                }
            }
            _ => return None,
        }
        Some(Self { arrange })
    }
}
