fn dangle() -> &String {
    let d = String::from("Hello");

    &d  // return the reference to d
}   // d goes out of scope here

fn main() {
    let s = dangle();
}