#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // println!("{:?}", four);
    // println!("{:?}", six);
    route(four);
    route(six);
}

fn route(ip: IpAddrKind) {
    println!("{:?}", ip);
}