fn main() {
    let num = if 1 > 2 {
        println!("if block");
        ()
    };

    // Use {:?} for printing special value (e.g structure, enum, etc)
    println!("{:?}", num);
}