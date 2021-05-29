use std::collections::HashMap;

fn main() {
    let scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 20);

    let score = scores.get("Blue");

    println!("{:?}, score");

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }
}