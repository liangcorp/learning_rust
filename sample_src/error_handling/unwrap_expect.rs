use std::fs::File;

fn main() {
    // panic with default message
    let f = File::open("abc.txt").unwrap();

    // panic with your message (e.g. Failed)
    let f2 = File::open("abc.txt").expect("Failed");
}