fn main() {
    // 5 is stored on the heap
    // Box is stored on the stack
    let b = Box::new(5);
    println!("{}", b);
}