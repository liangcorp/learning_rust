/*
    Move is for variable that uses heap:
        - Unknown size
        - Need dynamic allocating memory
        - E.g. String
*/

fn take(s1: String) {
    println!("{}", s1);
    //  s1 is dropped after the scope
}

fn main() {
    let s = String::from("Hello");

    take(s);

    // Won't work because s was moved then dropped after take(s);
    println!("{}", s);
}