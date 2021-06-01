/**
 * Author: Chen Liang
 * Description: Use thread channel for the following:
 *              - P1: Send message for return of borrowed money
 *              - P2: Received message and send message for returning
 *                      money.
 *              - P1: Received message for returning money
 */

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn ask_money(s: mpsc::Sender<&str>) {
    println!("Person 1: asking for money");
    s.send("money").unwrap();

    thread::sleep(Duration::from_millis(1));
}

fn return_money(s: mpsc::Sender<&str>, r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "money" {
        println!("Person 2: returning money");
        s.send("returned").unwrap();
    }
    thread::sleep(Duration::from_millis(1));
}

fn received_money(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "returned" {
        println!("Person 1: received money");
    }
    thread::sleep(Duration::from_millis(1));
}

fn main() {
    let (s_ch1, r_ch1) = mpsc::channel();
    let (s_ch2, r_ch2) = mpsc::channel();

    thread::spawn(|| {
        ask_money(s_ch1);
    });

    thread::spawn(|| {
        return_money(s_ch2, r_ch1);
    });

    let handle = thread::spawn(|| {
        received_money(r_ch2);
    });

    handle.join().unwrap();
}
