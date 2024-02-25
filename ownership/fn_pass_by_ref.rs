/*
    Pass by reference is pass a copy of pointer address value
    It's not passing the value that the pointer is pointing to
    The copy of pointer address is dropped after the scope finishes
    The original copy of pointer address is still there, and it is
    still pointing to the value.
*/

fn pass_by_ref(n: &mut i32)
{
    *n += 1;
    println!("n is {} in function", *n);
}

fn main() {
    let mut n: i32 = 10;

    println!("n is {} before calling pass_by_ref", n);

    pass_by_ref(&mut n);

    println!("n is {} after calling pass_by_ref", n);
}