use std::sync::{Mutex, Arc};
use std::thread;

static mut Seat: i32 = 1;

fn seat(name: &str) {
    unsafe {    // Needed for accessing static mutable variable
        if Seat >= 1 {
            println!("Seat available for {}", name);
            println!("Booking confirmed");
            println!("Printing ticket for {} ", name);
            Seat -= 1;
        } else {
            println!("Seat not available for {} ", name);
        }
    }
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let name = ["Peter", "Rob"];

    for i in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            let n = name[i];
            seat(n);
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}