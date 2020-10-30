fn main() {
    let some_u8_value: u8 = 1;
    match some_u8_value {
        _ => (),
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),

    }
}