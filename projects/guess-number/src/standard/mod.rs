use rand::{rngs::SmallRng, thread_rng, Rng, SeedableRng};

pub struct GuessingNumber {
    min: usize,
    max: usize,
    target: usize,
    rng: SmallRng,
}

impl GuessingNumber {
    pub fn new(min: usize, max: usize) -> Self {
        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
        Self { min, max, target: rng.gen_range(min..=max), rng }
    }
}

impl GuessingNumber {
    pub fn random_select(&mut self) -> usize {
        let mut n = 0;
        let mut min = self.min;
        let mut max = self.max;
        let mut guess: usize;
        loop {
            guess = self.rng.gen_range(min..=max);
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
        let mut max = self.max + 1;
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
        let mut max = self.max + 1;
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
