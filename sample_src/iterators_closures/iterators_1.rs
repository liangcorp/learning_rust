/*
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
 */

fn main() {
    let v1 = vec![1, 2, 3];

    // Produce iterator over immutable references
    let v1_iter = v1.iter();

    println!("{:?}", v1_iter);

    for val in v1_iter {    // Automatically cover to mutable reference
        println!("{}", val);
    }

    // empty because .next() consumes the iterator
    println!("{:?}", v1_iter);
}