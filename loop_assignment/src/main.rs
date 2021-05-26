use std::io;

fn is_even(a: &i32) -> bool {
    if *a % 2 == 0 {
        true
    } else {
        false
    }
}

fn find_digits_in_no(mut n: i32) -> i32 {
    let mut count: i32 = 0;

    while n != 0 {
        n = n / 10;
        count += 1;
    }

    count
}

fn main() {
    println!("Print even numbers from 1 to 100");
    for n in 1..=100 {
        if is_even(&n) {
            print!("{} ", n);
        }
    }
    println!("");

    let mut choice = String::new();

    for n in 0..3 {
        println!("What's 100 / 10?");

        choice.clear();
        io::stdin().read_line(&mut choice).expect("Failed");

        let answer: i32 = choice.trim().parse().expect("Failed");

        if answer == 10 {
            println!("{} is correct!", answer);
            break;
        } else {
            println!("Wrong answer. {} chance left.", 2 - n);
        }
    }

    println!("Number of digits in 12345 is {}",
                                find_digits_in_no(12345));
}

