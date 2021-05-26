fn sub_add(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

fn main() {
    // sub_add returns a tuple. "{}" won't work on compound types.
    println("{:?}", sub_add(4, 2));
}