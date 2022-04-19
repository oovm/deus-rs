use super::*;

impl Debug for NQueensBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.arrange)
    }
}

impl Display for NQueensBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let n = self.arrange.len();
        for i in self.arrange.iter().rev() {
            for j in 1..=n {
                if *i == j {
                    write!(f, " Q")?;
                }
                else {
                    write!(f, " .")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
