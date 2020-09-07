fn main() {
    let nth_digit: i32 = 100;
    let mut a:i128 = 0;
    let mut b = 1;
    for _ in 0..nth_digit {
        println!("{}", b);
        let tmp = a;
        a = b;
        b = a + tmp;
    }
}