fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;    // s1 is dropped after assignment

    println!("{}", s2);

    // Won't work because value of "s" had been moved
    println!("{} {}", s1, s2);
}