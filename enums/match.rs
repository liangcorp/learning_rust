enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Arizona,
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);    // Funky Rust bind action
            25
        },
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
}