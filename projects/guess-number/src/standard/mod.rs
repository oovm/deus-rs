// target = RandomInteger[{1, 10000}]
// {min, max} = {1, 10000}
// n = 0;
// While[
//  True,
//  guess = RandomInteger[{min, max}];
//  n += 1;
//  Which[
//   guess > target, max = guess,
//   guess < target, min = guess,
//   True, Return[guess]
//   ]
//  ]
// {n, guess, target}

use rand::random;

const min = 1;
const max = 10000;


fn random_select() -> usize {
    let mut n = 0;
    let target = random();
    let mut guess = random();
    loop {
        guess = random();
        n += 1;
        if guess > target {
            max = guess;
            continue;
        }
        if guess < target {
            min = guess;
            continue
        }
        break
    }
    n
}

fn golden_divide() {

}

fn dichotomy() {

}
