extern crate num;
use num::Num;

trait Addition {
    fn add<T: Num>(&self, num1: T, num2: T) -> T;
}

struct Numbers <T: Num> {
    x: T,
    y: T,
}

impl <T: Num> Addition for Numbers <T> {
    fn add<U: Num>(&self, num1: U, num2: U) -> U {
        num1 + num2
    }
}

/*
fn add<T: Num>(num1: T, num2: T) -> T {
    num1 + num2
}
 */

fn main() {
    let sum = Numbers {
        x: 3,
        y: 4,
    };
    println!("Sum of {} and {} is {}", sum.x, sum.y,
                                        sum.add(sum.x, sum.y));
}
