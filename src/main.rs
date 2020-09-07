use std::process::exit;

fn main() {

    let nth_digit: i32 = 100;
    let mut f_0: i128 = 1;  // f_n
    let mut f_1 = 1;  // f_n+1
    let mut index = 2;
    println!("{}", 1);
    println!("{}", 1);

    if nth_digit == 1 || nth_digit == 2 {
        println!("1");
        exit(0);
    }
    while index < nth_digit {
        let tmp = f_0;
        f_0 = f_1;
        f_1 = tmp + f_1;
        index += 1;
        println!("{}", f_1)
    }


}