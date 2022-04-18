use super::*;

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate() {
            true => self.fmt_pretty(f),
            false => self.fmt_simple(f),
        }
    }
}

impl SudokuBoard {
    fn fmt_simple(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.filled().iter().enumerate() {
            if i % self.rank.pow(2) == 0 {
                write!(f, "\n")?;
            }
            write!(f, " {} ", n)?;
        }
        Ok(())
    }
    fn fmt_pretty(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.filled().iter().enumerate() {
            if i % self.rank.pow(2) == 0 {
                write!(f, "\n")?;
            }
            else if i % self.rank == 0 {
                write!(f, "|")?;
            }
            write!(f, " {} ", n)?;
        }
        Ok(())
    }
}
