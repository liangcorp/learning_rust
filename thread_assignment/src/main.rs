/**
 * Author: Chen Liang
 * Description: Use thread channel to do the following:
 *              - Factory produces car
 *              - It then send message to paint car
 *              - It then send message to sell car
 * Date: 01-06-2021
 */

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn produce_car(s: mpsc::Sender<&str>) {
    for i in 1..=10 {
        println!("Produced car {}", i);
    }
    thread::sleep(Duration::from_millis(1));
    s.send("Produced").unwrap();
}

fn paint_car(s: mpsc::Sender<&str>) {
    for i in 1..=10 {
        println!("Pained car {}", i);
    }
    thread::sleep(Duration::from_millis(1));
    s.send("Painted").unwrap();
}

fn sell_car(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "Produced"
                            && r.recv().unwrap() == "Painted" {
        println!("Selling car");
    }
}

fn main() {
    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);

    let handle = thread::spawn(|| {
        produce_car(s);
        paint_car(s1);
        sell_car(r);
    });

    handle.join().unwrap();
}
