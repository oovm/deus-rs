use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

#[test]
fn test() {
    (0..100)
        .into_par_iter()
        .map(|_| {
            let game = GuessingNumber::new(1, 100);
            let a = game.random_select();
            let b = game.golden_divide();
            let c = game.dichotomy();
        })
        .collect::<Vec<_>>();
    for _ in 0..100 {
        let a = game.random_select();
        let b = game.golden_divide();
        let c = game.dichotomy();
    }

    //  println!("{}", game.golden_divide());
    //   println!("{}", game.dichotomy());
}

pub struct GuessingNumber {
    min: usize,
    max: usize,
    target: usize,
    rng: SmallRng,
}

impl GuessingNumber {
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max, target: thread_rng().gen_range(min..=max) }
    }
}

impl GuessingNumber {
    pub fn random_select(&self) -> usize {
        let mut n = 0;
        let mut min = self.min;
        let mut max = self.max;
        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
        let mut guess: usize;
        loop {
            guess = rng.gen_range(min..=max);
            n += 1;
            if guess > self.target {
                max = guess;
                continue;
            }
            if guess < self.target {
                min = guess;
                continue;
            }
            break;
        }
        n
    }
    pub fn golden_divide(&self) -> usize {
        let mut n = 0;
        let mut min = self.min;
        let mut max = self.max;
        let mut guess: usize;
        loop {
            guess = (max as f32 * 0.618034 + min as f32 * 0.381966) as usize;
            n += 1;
            if guess > self.target {
                max = guess;
                continue;
            }
            if guess < self.target {
                min = guess;
                continue;
            }
            break;
        }
        n
    }
    pub fn dichotomy(&self) -> usize {
        let mut n = 0;
        let mut min = self.min;
        let mut max = self.max;
        let mut guess: usize;
        loop {
            guess = (min + max) / 2;
            n += 1;
            if guess > self.target {
                max = guess;
                continue;
            }
            if guess < self.target {
                min = guess;
                continue;
            }
            break;
        }
        n
    }
}
