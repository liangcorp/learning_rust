fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(" World");

    let s3 = s1 + &s2;      // be aware that s1 is now moved
    println!("{}", s3);
}