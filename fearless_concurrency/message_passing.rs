use std::thread;
use std::sync::mpsc;

fn main() {
    let (s, r) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        s.send(val).unwrap();
    });

    // Blocking main thread execution until receiving message
    let rec = r.recv().unwrap();

    println!("Got {}", rec);
}