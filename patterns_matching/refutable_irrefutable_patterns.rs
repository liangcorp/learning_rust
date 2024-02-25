fn main() {
    // Refutable
    // Irrefutable
    let value: Option<i32> = None;
    let Some(x) = value;    // Not going to work

    if let Some(x) = value {    // Works

    }
}