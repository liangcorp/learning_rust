fn main() {
    let a = 100;
    let b = 20;
    let c = 50;

    if a > b && a > c {
        println!("A is the largest");
    } else if b > a && b > c {
        println!("B is the largest");
    } else {
        println!("C is the largest");
    }
}