/*
    Move is for variable that uses heap:
        - Unknown size
        - Need dynamic allocating memory
        - E.g. String
*/

fn take_return(s1: String) -> String {
    println!("{}", s1);

    s1  // return s1 (i.e. move it back)
}

fn main() {
    let mut s = String::from("Hello");

    s = take_return(s);

    println!("{}", s);  // works because s was returned (moved back)
}