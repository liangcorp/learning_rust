#[derive(Debug)]

struct User {
    age: i32
}

fn main() {
    let mut u1 = User {age: 25};
    u1.age = 26;

    let u2 = User {age: 30};
    println!("{:?}", u1.age);

    if u1.age > u2.age {
        println!("u1 is elder");
    } else {
        println!("u2 is elder");
    }
}