fn main() {
    let p1 = (2, 4);
    let p2 = (5.0, 9.0);

    println!(
        "The absolute difference between p1 and p2 is {:?}",
        ((p1.0 as f64 - p2.0).abs(), (p1.1 as f64 - p2.1).abs())
    );

    let p2 = [5.0, 9.0];
    let p1 = [2, 4];

    println!(
        "The absolute difference between p1 and p2 is {:?}",
        ((p1[0] as f64 - p2[0]).abs(), (p1[1] as f64 - p2[1]).abs())
    );

    let p1: (f64, f64) = (4.0, 3.0);
    let p2: (f64, f64) = (5.0, 4.5);

    println!("The distance between two points is {}", (p1.0 - p2.0).powf(2.0) + (p1.1 - p2.1).powf(2.0));

    let a: i32 = -15;
    let b: i32 = 170;
    let my_name: &str = "Michael";

    println!("My name is {}, and the multiplication result is {}", my_name, a * b);
}
