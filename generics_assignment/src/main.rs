use generics_assignment::Numbers;
use generics_assignment::Addition;
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
