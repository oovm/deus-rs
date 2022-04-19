use crate::GuessingNumber;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use wolfram_library_link::{
    export,
    expr::{Expr, Number, Symbol},
};

fn failed() -> Expr {
    Expr::symbol(Symbol::new("System`$Failed"))
}

#[export(wstp)]
fn guess_number_ffi(args: Vec<Expr>) -> Expr {
    match args.as_slice() {
        [a, b, c] => {
            let min = match a.try_number().unwrap() {
                Number::Integer(i) if i >= 0 => i as usize,
                _ => return failed(),
            };
            let max = match b.try_number().unwrap() {
                Number::Integer(i) if i as usize > min => i as usize,
                _ => return failed(),
            };
            let times = match c.try_number().unwrap() {
                Number::Integer(i) if i > 0 => i as usize,
                _ => return failed(),
            };
            Expr::list(guess_number(min, max, times))
        }
        _ => failed(),
    }
}

fn guess_number_dichotomy(min: usize, max: usize, times: usize) -> Vec<Expr> {
    (0..times)
        .into_par_iter()
        .map(|_| {
            let mut game = GuessingNumber::new(min, max);
            let a = game.dichotomy();
            let b = game.golden_divide();
            let c = game.random_select();
            Expr::list(vec![Expr::from(a as i32), Expr::from(b as i32), Expr::from(c as i32)])
        })
        .collect()
}

#[test]
fn test() {}
