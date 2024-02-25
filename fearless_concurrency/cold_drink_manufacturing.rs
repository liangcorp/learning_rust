use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn bottle(s: mpsc::Sender<&str>) {
    for i in 1..6 {
        println!("Building bottle {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("Bottles are ready");
    s.send("Bottle").unwrap();
}

fn cold_drink(s: mpsc::Sender<&str>) {
    let mut i = 5;
    while i > 0 {
        println!("Creating cold drink . Hours left: {}", i);
        thread::sleep(Duration::from_micros(1));
        i = i - 1;
    }
    println!("Cold drink is ready");
    s.send("Cold").unwrap();
}

fn seller(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "Bottle" && r.recv().unwrap() == "Cold" {
        println!("Thanks for doing on time");
        println!("I will sell cold drink now");
    }
}

fn main() {
    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);

    let handle = thread::spawn(|| {
        bottle(s1);
        cold_drink(s);
        seller(r);
    });

    handle.join().unwrap();
}