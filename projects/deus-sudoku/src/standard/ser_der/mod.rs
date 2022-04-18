use super::*;

impl FromStr for SudokuBoard {
    type Err = &'static str;
    /// rank 3: 0-9
    /// rank 4: 0-F
    /// rank 5: 0-?
    /// rank 6: 0-z
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = vec![];
        for c in s.chars() {
            let n = match c {
                ' ' | '\r' => continue,
                '.' | '*' | '-' | '_' => 0,
                '0'..='9' => c as usize - b'0' as usize,
                'a'..='z' => c as usize - b'a' as usize + 10,
                'A'..='Z' => c as usize - b'A' as usize + 10,
                _ => return Err("invalid character"),
            };
            out.push(n as usize);
        }
        Ok(SudokuBoard::new(out))
    }
}

macro_rules! from_str {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SudokuBoard {
                fn from(input: $t) -> Self {
                    SudokuBoard::from_str(&input).unwrap()
                }
            }
        )*
    };
}

from_str![&str, &String, String];

impl From<Sudoku3> for SudokuBoard {
    fn from(game: Sudoku3) -> Self {
        Self { rank: 3, state: game.to_bytes().iter().map(|s| *s as usize).collect() }
    }
}

impl From<&SudokuBoard> for Sudoku3 {
    fn from(game: &SudokuBoard) -> Self {
        debug_assert_eq!(game.rank, 3);
        Sudoku3::from_bytes_slice(&game.filled_u8()).unwrap()
    }
}

impl SudokuBoard {
    pub(crate) fn filled(&self) -> Vec<usize> {
        self.state.iter().cloned().chain([0usize].iter().cycle().cloned()).take(self.rank.pow(4)).collect()
    }
    pub(crate) fn filled_u8(&self) -> Vec<u8> {
        self.state.iter().map(|x| *x as u8).chain([0u8].iter().cycle().cloned()).take(self.rank.pow(4)).collect()
    }
}
