fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{} {}", s1, s2);

    println!("{}", s3);

    // Format! macro doesn't take ownership
    println!("{}", s1);
}