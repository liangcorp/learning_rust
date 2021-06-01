use std::thread;
use std::sync::{Mutex, Arc};
use std::vec::Vec;

static mut NO_OF_CAB: i32 = 2;

fn book_cab(name: &str) {
    unsafe {
        if NO_OF_CAB >= 1 {
            println!("{} cab available.", NO_OF_CAB);
            println!("Booking confirmed for {}", name);
            NO_OF_CAB -= 1;
        } else {
            println!("Cab not available for {}", name);
        }
    }
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    let name = ["Peter", "Rob", "Alice", "John"];

    for i in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            let n = name[i];
            book_cab(n);
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
