fn largest<T: PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn main() {
    let list = vec![1, 2, 3, 4];
    let result = largest(&list);
    println!("{}", result);

    let list = vec!['y', 't', 'u'];
    let result = largest(&list);
    println!("{}", result);
}