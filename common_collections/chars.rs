fn main() {
    let s1 = String::from("Hello");

    let n = &s1[0..1];

    for n in s1.chars() {
        println!("{}", n);
    }
}