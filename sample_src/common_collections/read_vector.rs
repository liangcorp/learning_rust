fn main() {
    let v = vec![1, 2, 3];
    let value = v[0];       // Returning the value

    let value2 = v.get(0);  // Returning Some(0);

    println!("{:?}", value2);
}