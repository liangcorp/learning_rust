use std::sync::mpsc;
use std::thread;
use std::time::Duration;
/**
 *  Author: Chen Liang
 *  Description: Use thread to simulate the following:
 *                  Factory create car body and engin, then it send
 *              messages to assemble the car. Then it send message
 *              to the seller to sell the car
 *  Date: 1st-Jun-2021
 */

fn create_body(s: mpsc::Sender<&str>) {
    for i in 1..=10 {
        println!("Built body {}", i);
    }
    s.send("body").unwrap();
    thread::sleep(Duration::from_millis(1));
}

fn create_engin(s: mpsc::Sender<&str>) {
    for i in 1..=10 {
        println!("Built engin {}", i);
    }
    s.send("engin").unwrap();
    thread::sleep(Duration::from_millis(1));
}

fn assemble_car(s: mpsc::Sender<&str>, r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "body" && r.recv().unwrap() == "engin" {
        println!("Assemble cars");
        println!("Ready to sell");
        s.send("sell").unwrap();
    }
    thread::sleep(Duration::from_millis(1));
}

fn sell_car(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "sell" {
        println!("Selling car.");
    }
    thread::sleep(Duration::from_millis(1));
}

fn main() {
    let (s_ch1, r_ch1) = mpsc::channel();
    let s_ch1_engin = mpsc::Sender::clone(&s_ch1);

    let (s_ch2, r_ch2) = mpsc::channel();

    thread::spawn(|| {
        create_body(s_ch1);
        create_engin(s_ch1_engin);
        assemble_car(s_ch2, r_ch1);
    });

    let handle = thread::spawn(|| {
        sell_car(r_ch2);
    });

    handle.join().unwrap();
}
