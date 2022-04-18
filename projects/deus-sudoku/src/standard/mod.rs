use std::{convert::TryInto, str::FromStr};

mod display;
mod parse;

pub struct Sudoku {}

impl Sudoku {}

#[derive(Clone)]
pub struct SudokuBoard {
    rank: usize,
    state: Vec<usize>,
}

impl Sudoku {
    pub fn is_solvable<G>(game: G) -> bool
    where
        G: Into<SudokuBoard>,
    {
        game.into().is_solvable()
    }

    pub fn is_uniquely_solvable<G>(game: G) -> bool
    where
        G: Into<SudokuBoard>,
    {
        game.into().is_uniquely_solvable()
    }
}

impl SudokuBoard {
    pub(crate) fn filled(&self) -> Vec<usize> {
        self.state.iter().chain([0usize].iter().cycle()).take(self.rank.pow(4)).collect()
    }
    pub(crate) fn filled_u8(&self) -> Vec<u8> {
        self.state.iter().map(|x| *x as u8).chain([0u8].iter().cycle()).take(self.rank.pow(4)).collect()
    }
}

impl SudokuBoard {
    pub fn new(v: Vec<usize>) -> Self {
        let rank = v.len().sqrt().sqrt();
        assert_eq!(rank.pow(4), v.len());
        Self { rank, state: v }
    }
    pub fn new_with_rank(v: Vec<usize>, rank: usize) -> Self {
        Self { rank, state: v }
    }
}

impl SudokuBoard {
    pub fn is_solvable(&self) -> bool {
        match self.rank {
            3 => {
                let array: [u8; 81] = self.filled_u8().try_into().unwrap();
                let mut solver = sudoku::Sudoku(array);
                solver.solve_one().is_some()
            }
            _ => unimplemented!(),
        }
    }
    pub fn is_unique_solvable(&self) -> bool {
        match self.rank {
            3 => {
                let array: [u8; 81] = self.filled_u8().try_into().unwrap();
                let mut solver = sudoku::Sudoku(array);
                solver.is_uniquely_solvable()
            }
            _ => unimplemented!(),
        }
    }
}

pub fn generate(rank: usize) -> SudokuBoard {
    SudokuBoard { rank, state: vec![] }
}

impl FromStr for SudokuBoard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = vec![];
        for c in s.as_bytes() {
            let n = match c {
                b'.' | b'*' | b'-' | b'_' => 0,
                b'0'...b'9' => c - b'0',
                b'a'...b'z' => c - b'a' + 10,
                b'A'...b'Z' => c - b'A' + 10,
                _ => return Err("invalid character"),
            };
            out.push(n as usize);
        }
        Ok(SudokuBoard::new(out))
    }
}

#[test]
fn test() {
    let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

    let sudoku = SudokuBoard::from_str(sudoku_line).unwrap();

    println!("{}", sudoku.is_solvable());
    println!("{}", sudoku.is_unique_solvable());
}
