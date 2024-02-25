use std::thread;
use std::time::Duration;

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

let add_one_v2 = |x: u32| -> u32 {
    x + 1
};

/**
 * You can only assign one type to the closure
 * (the first time it's used)
 */

let add_one_v4 = |x| {
    // Unused. Compilations error!
    // Complies fine if used once
    x + 1
};

// Unused. Compilations error!
// Complies fine if used once
let add_one_v5 = |x| x + 1 ;

fn main() {
    // Used. Assuming u32 type
    let expensive_closure = |num| {
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let x = expensive_closure(2);
    println!("{}", x);
}