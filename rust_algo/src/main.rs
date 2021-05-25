fn main() {
    let number = 13;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
}
