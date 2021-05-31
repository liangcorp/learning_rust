fn main() {
    let x = vec![4];
    let equal = move |z| z == x ;

    println!("{:?}", x);    // Compilation Error: x moved

    let y = vec![4];

    assert_eq!(equal(y));
}