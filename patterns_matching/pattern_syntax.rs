fn main() {
    let x = 1;
    let _a = 1; // '_' allows no warning even never used

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1...5 => println!("one through five"),
        _ => println!("Something else"),
    }

    match x {
        'a'...'j' => println!("a to j"),
        _ => println!("Something else"),
    }

    for i in 1..=5 {
        println!("{}", i);
    }
}