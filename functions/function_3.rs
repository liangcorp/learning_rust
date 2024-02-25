fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add(2, 3);

    println!("{}", sum);
    println!("{}", add(4, 5));
}