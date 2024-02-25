#[derive(Debug)]

struct User {
    name: String,
    age: i32
}

fn build(name: String, new_age: i32) -> User {
    User {
        name: name,
        age: new_age
    }
}

fn build2(name: String, age: i32) -> User {
    User {
        name: name,
        age: age
    }
}
// can also be written like the following
fn build3(name: String, age: i32) -> User {
    User {
        name,
        age
    }
}


fn main() {
    let u1 = build(String::from("Rob"), 20);
    println!("{:?}", u1);
}