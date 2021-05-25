use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<char> = input.chars().collect();
    println!("{}{}{}", v[1], v[2], v[0]);
}

#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}