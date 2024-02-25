fn print(s1: &String) {
    println!("{}", s1);
}

fn main() {
    let s = String::from("Hello");
    print(&s);
    println!("{}", s);
}