/*
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
 */

fn main() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut() // Return mutable reference

    println!("{:?}", v1_iter);

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);

    // empty because .next() consumes the iterator
    println!("{:?}", v1_iter);
}