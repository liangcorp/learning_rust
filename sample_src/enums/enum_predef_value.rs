#[derive(Debug)]
enum Fruits {
    Apple = 0,
    Mango = 10,
    Watermelon = 20,
}

fn main() {
    let f = Fruits::Mango;
    println!("{:?}", f);        // Mango
    println!("{:?}", f as i32); // 10
}