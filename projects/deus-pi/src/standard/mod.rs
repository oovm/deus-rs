use rug::float::Round;
use rug::ops::DivAssignRound;
use rug::ops::Pow;
use rug::Float;

use rug::Rational;
use rug::Integer;
use core::cmp::Ordering;
use dashu::{Integer, Rational, Real};

fn _rational_to_float(ration: &Rational ,precision: u32) -> Float {
    let numer = ration.numer();
    let denom = ration.denom();
    let mut fl = Float::with_val(precision, numer);
    let _ = fl.div_assign_round(denom, Round::Zero);
    fl
}

fn _leibniz_rational(precision: u32) -> Float {
    let mut pi = Rational::from((0,1));
    let mut i = Integer::from(0);
    let four = Integer::from(4);
    let zero = Integer::from(0);

    loop {
        if i.mod_u(175000) == 0 {
            let pi_float = _rational_to_float(&pi, precision);
            match pi_float.get_exp() {
                Some(num) if num > precision as i32 => break,
                _ => (),
            }
            println!("pi: {}, iteration: {}", pi_float, i);
        }
        let next_denom = Integer::from(&i * 2) + Integer::from(1);
        let next_rational = Rational::from((&four, &next_denom));
        if Integer::from(&i % 2).cmp(&zero) == Ordering::Equal {
            pi += next_rational;
        } else {
            pi -= next_rational;
        }

        i += 1;
    }

    _rational_to_float(&pi, precision)
}

fn _leibniz(precision: u32) -> Float {
    let _zero = Real::with_val(precision, 0);
    let mut pi = Real::with_val(precision, 0);
    let mut i = 0;

    loop {
        let mut next = Real::with_val(precision, 4);
        let next_div = Real::with_val(precision, i * 2 + 1);
        let _next_dir = next.div_assign_round(next_div, Round::Zero);
        if i % 175000 == 0 {
            let exp = next.get_exp().unwrap();
            if exp.abs() > precision as i32 {
                break;
            }

            //println!("{}, {}", pi, next);
            println!("pi: {}, iteration: {}", pi, i);
        }

        if i % 2 == 0 {
            pi += next;
        } else {
            pi -= next;
        }
        i += 1;
    }
    pi
}

// https://en.wikipedia.org/wiki/Chudnovsky_algorithm
fn _chudnovski(precision: u32) -> Real {
    let mut big_k = Integer::from(6);
    let mut x = Integer::from(1);
    let mut m = Integer::from(1);
    let mut l = Integer::from(13591409);
    let mut sum = Real::with_val(precision, 13591409);
    let mut small_k = Integer::from(0);

    loop {
        let k_3 = Integer::from((&big_k).pow(3));
        m = Integer::from(&k_3 - (16 * &big_k)) * &m / &k_3;
        l += 545140134;
        x *= -262537412640768000_i64;
        sum += Real::with_val(precision, Integer::from(&m * &l) / &x);
        big_k += 12;
        small_k += 1;

        if small_k > 100000 {
            break;
        }
    }

    let c = Real::with_val(precision, 10005).root(2) * 426880;
    let pi : Real = &c / sum;
    //println!("{}", c);
    pi
}

#[test]
fn main() {
    let precision = 100;
    let _one = Real::with_val(precision, 1);
    let pi = _chudnovski(precision);

    let pi_string = pi.to_string_radix(10, None);
    let mut pi_string = pi.to_string_radix(10, Some(pi_string.len() - 3));
    pi_string.pop();
    println!("{}", pi_string);
}
