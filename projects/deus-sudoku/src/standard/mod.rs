use num::integer::Roots;
use std::str::FromStr;
use sudoku::Sudoku as Sudoku3;

mod display;
mod ser_der;

#[derive(Clone)]
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
    #[track_caller]
    pub fn new(v: Vec<usize>) -> Self {
        let rank = v.len().sqrt().sqrt();
        assert_eq!(rank.pow(4), v.len());
        Self { rank, state: v }
    }
    pub fn new_with_rank(v: Vec<usize>, rank: usize) -> Self {
        Self { rank, state: v }
    }
    #[track_caller]
    pub fn generate(rank: usize) -> Self {
        match rank {
            3 => Self::from(Sudoku3::generate_filled()),
            _ => unimplemented!(),
        }
    }
    #[track_caller]
    pub fn generate_uniquely(rank: usize) -> Self {
        match rank {
            3 => Self::from(Sudoku3::generate_unique()),
            _ => unimplemented!(),
        }
    }
    pub fn is_solvable(&self) -> bool {
        match self.rank {
            3 => Sudoku3::from(self).solve_one().is_some(),
            _ => unimplemented!(),
        }
    }
    pub fn is_uniquely_solvable(&self) -> bool {
        match self.rank {
            3 => Sudoku3::from(self).is_uniquely_solvable(),
            _ => unimplemented!(),
        }
    }
    pub fn solve(&self) -> Self {
        match self.rank {
            3 => Self::from(Sudoku3::generate_unique()),
            _ => unimplemented!(),
        }
    }
}

#[test]
fn test() {
    let sudoku_line = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

    let sudoku = SudokuBoard::from_str(sudoku_line).unwrap();

    println!("{}", sudoku.is_solvable());
    println!("{}", sudoku.is_uniquely_solvable());
}
