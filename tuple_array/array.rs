fn print_array(x: [i32; 5]) {
    for n in 0..5 {
        println!("{}", a[n]);
    }

    for n in x.iter() {
        println!("{}", n);
    }

    println!("Length of the array is {}", x.len());
}

fn main() {
    let a = [22, 34, 65, 3];
    let a: [i32; 4] = [22, 34, 65, 3];

    let a: [i32; 5] = [0; 5]        // [0, 0, 0, 0, 0]
    let mut a: [i32; 5] = [3; 5]    // [3, 3, 3, 3, 3]
    a[0] = 1;
    println!("{:?}", a);

    print_array(a);
}