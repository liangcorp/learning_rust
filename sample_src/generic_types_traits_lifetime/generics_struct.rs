#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

struct MultiPoint<T, E> {
    x: T,
    y: E,
}

impl <T> Point <T> {
    fn x (&self) -> &T {
        &self.x
    }
}

fn main() {
    let integer = Point{x: 5, y: 10};
    let float = Point{x: 8.0, y:9.4};

    let muli_type = MultiPoint{x: 5, y: 10.01};

    println!("{:?}\n{:?}", integer.x, float);
}