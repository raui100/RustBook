struct Fibonacci(u128, u128);

fn next_f(f: &Fibonacci) -> Option<Fibonacci> {
    let f_1 = f.1.checked_add(f.0);
    match f_1 {
        None => None,
        Some(f_1) => Some(Fibonacci(f.1, f_1))
    }
}

fn main() {
    let mut f = Some(Fibonacci(0, 1));
    loop {
        match &f {
            None => break,
            Some(fib) => {
                println!("{}", fib.0);
                f = next_f(&fib);
            }
        }
    }
}
