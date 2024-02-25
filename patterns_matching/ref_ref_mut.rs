fn main() {
    let mut name = Some(String::from("Rob"));

    match name {
        Some(ref name) => println!("Found name {}", name);
        None => (),
    }

    match name {
        Some(ref mut name) => *name = String::from("ABC"),
        None => (),
    }

    println!("{:?}", name);
}