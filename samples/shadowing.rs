/*
    shadowing a variable
    - Different than mut
    - Can change the type of the value but reuse the same name
*/

fn main() {
    let a = 10; //  WARNING: won't be used

    let a = 20;
    println!("{}", a);

    a = 30;             // Won't compile
    let a: i32 = 30;    // Works

    a = 'A';            // Won't compile
    let a: char = 'A';  // Works


    println!("{}", a);
}