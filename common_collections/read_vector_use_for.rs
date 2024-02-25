fn main() {
    let v = vec![1, 2, 3, 4];

    for i in &v {
        *i = *i + 2;
        println!("{}", i);
    }
}