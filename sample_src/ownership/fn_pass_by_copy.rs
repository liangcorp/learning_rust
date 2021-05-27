/*
    Pass by copy is for variable that uses stack:
        - Know size
        - Fixed memory allocation
        - E.g. i32, f64, bool, etc
*/

fn pass_by_copy(mut n: i32)
{
    n += 1;
    println!("n is {} in function", n);
}

fn main() {
    let n = 10;

    println!("n is {} before calling pass_by_copy", n);

    pass_by_copy(n);

    println!("n is {} after calling pass_by_copy", n);
}