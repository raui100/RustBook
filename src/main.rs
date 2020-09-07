fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n -1) + fibonacci(n -2),
    }
}
fn main() {
    for n in 0..10 {
        println!("{}", fibonacci(n));
    }
}