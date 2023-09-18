#[derive(Debug)]

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let mut node1 = Node {
        data: 1,
        next: None,
    };

    let mut node2 = Node {
        data: 2,
        next: Some(Box::from(node1)),
    };

    let mut node3 = Node {
        data: 3,
        next: Some(Box::from(node2)),
    };

    println!("{:?}", node3);
}
