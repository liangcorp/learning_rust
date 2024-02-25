fn main() {
    let x = 4;
    let equal = |z| z == x ;    // Can access x while in scope

    fn equal(z: i32) -> bool {
        z == x  // Can't access x (out of scope)
    }

    let y = 4;

    assert_eq!(equal(y));
}