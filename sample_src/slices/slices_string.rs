/*
    Slices let you reference a contiguous sequence of elements
*/

fn main() {
    let a = String::from("Hello_World");

    let r1 = &a[0..5];
    println!("{}", r1);

    let r2 = &a[0..=5];
    let r2 = &a[..=5];  // same as above
    println!("{}", r2);

    let r3 = &a[0..];
    println!("{}", r3);

    let r4 = &a[..];
    println!("{}", r4);
}