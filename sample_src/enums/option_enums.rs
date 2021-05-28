
fn main() {
    let num = Some(5);
    let text = Some("Hello");

    let num2: Option<i32> = None;
    let text2: Option<&str> = None;

    println!("{:?} {:?}", num, text);
    println!("{:?} {:?}", num2, text2);
}