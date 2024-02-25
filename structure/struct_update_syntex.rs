#[derive(Debug)]

struct User {
    name: String,
    age: i32,
    hobby: String,
}

fn main() {
    let u1 = User {
        name: String::from("Rob"),
        age: 25,
        hobby: String::from("Cricket"),
    };

    let u2 = User {
        name: String::from("Bob"),
        age: u1.age,
        hobby: u1.hobby.clone(),
    };

    let u3 = User {
        name: String::from("Alice"),
        ..u1    // Won't work for String
    }

    println!("u1 {:?}\nu2 {:?}", u1, u2);
}