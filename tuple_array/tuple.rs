fn print_tuple(x: (i32, bool, f64)) {
    println!("{:?}", x);

    let (a, y, z) = x;
    println!("{} {} {}", a, y, z);
}

fn main() {
    /*
        Tuple can't change size
        Can contain different types of variables
    */
    let a = (220, true, 8.5);
    let a: (i32, bool, f64) = (220, true, 8.5);

    println!("{:?}", a);
    println!("{}", a.0);
    println!("{}", a.1);
    println!("{}", a.2);

    print_tuple(a);
}