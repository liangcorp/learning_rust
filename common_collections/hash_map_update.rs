use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();
    score.insert("Blue", 10);
    score.entry("Blue").or_insert(20);
    score.entry("Red").or_insert(30);

    println!("{:?}", score);
}