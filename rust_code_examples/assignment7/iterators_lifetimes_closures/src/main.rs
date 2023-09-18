fn main() {
    let mut vec_1: Vec<u32> = vec![5, 4, 3, 6, 9];
    let mut vec_2: Vec<u32> = vec![5, 8, 6, 4, 10, 15, 20, 21];

    let result = intersection(&mut vec_1, &mut vec_2);
    println!("Intersection: {:?}", result);

    let result = union(&mut vec_1, &mut vec_2);
    println!("Union: {:?}", result);
}

fn intersection(vec_1: &mut [u32], vec_2: &mut [u32]) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];

    vec_1.iter().for_each(|x| {
        if vec_2.iter().any(|&y| y == *x) {
            result.push(*x)
        }
    });

    result
}

fn union(vec_1: &mut [u32], vec_2: &mut [u32]) -> Vec<u32> {
    let mut result: Vec<u32> = vec_2.to_owned();

    vec_1.iter().for_each(|x| {
        if !vec_2.iter().any(|&y| y == *x) {
            result.push(*x)
        }
    });

    result
}
