/*
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
 */

fn main() {
    let v1 = vec![1, 2, 3];

    // Produce iterator over mutable references
    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter);

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // empty because .next() consumes the iterator
    println!("{:?}", v1_iter);
}