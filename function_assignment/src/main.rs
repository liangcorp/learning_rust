use std::io;

fn factorial (mut n: i32) -> i32 {
    let mut facto: i32 = 1;

    while n != 0 {
        facto = facto * n;
        n -= 1;
    }

    facto
}

fn main() {

    let mut number = String::new();

    println!("Enter a integer: ");

    io::stdin().read_line(&mut number).expect("Failed");

    let number: i32 = number.trim().parse().expect("Failed");

    println!("Factorial of {} is {}", number, factorial(number));
}
