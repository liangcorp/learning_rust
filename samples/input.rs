use std::io;

fn main() {
    let mut a = String::new();
    println!("Enter a string:");

    io::stdin().read_line(&mut a).expect("Failed");

    // Trim the return character
    let a: String = a.trim().parse().expect("Failed");

    println!("\"{}\" is what you entered", a);
}