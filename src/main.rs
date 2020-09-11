use num::BigUint;
use num::{Zero, One};
use std::mem::replace;

// Calculates large fibonacci numbers.
fn fib(n: u128) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn main() {
    println!("fib({}) = {}", 100000, fib(100000));
}