struct Point<T> {
    x: T,
}

impl Point<f32> {
    fn number(&self) -> f32 {
        self.x
    }
}

impl Point<i32> {
    fn number(&self) -> i32 {
        self.x
    }
}

fn main() {
    let n = Point{x: 2};
    println!("{}", n.number());
    let n1 = Point{x: 2.2};
    println!("{}", n1.number());
}