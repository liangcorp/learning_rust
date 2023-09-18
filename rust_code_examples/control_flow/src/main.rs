fn main() {
    // Question 1
    println!("Enter a number");
    let mut num = String::new();

    std::io::stdin()
        .read_line(&mut num)
        .expect("failed to read input");

    let num: i32 = num.trim().parse().expect("invalid input");

    let mut sum: i32 = 0;

    for i in 1..=num {
        sum += i;
    }

    let square_of_sum = sum.pow(2);

    sum = 0;

    for i in 1..=num {
        sum += i.pow(2);
    }

    let difference = square_of_sum - sum;

    println!("Difference is {}", difference);

    // Question 2

    let mut num = String::new();

    std::io::stdin().read_line(&mut num).expect("input error");

    let num: i32 = num.trim().parse().expect("parcing error");

    let mut sum: i32 = 0;

    for i in 1..num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("Sum is {}", sum);

    // Question 3

    // Question 4
    if is_palindrome("TiaciT") {
        println!("is palindrome");
    } else {
        println!("Not palindrome");
    }

    // Question 5
    println!("{:?}", pythagorean_triplet());
}

#[allow(dead_code)]
fn total_production(hours: i32, speed: u8) -> i32 {
    if speed > 10 {
        println!("invalid speed");
    }
    match speed {
        1..=4 => 221 * hours,
        5..=8 => (221_f64 * 0.9 * hours as f64) as i32,
        9..=10 => (221_f64 * 0.77 * hours as f64) as i32,
        _ => 0,
    }
}

#[allow(dead_code)]
fn cars_production_per_minute(minutes: i32, speed: u8) -> i32 {
    if speed > 10 {
        println!("invalid speed");
    }
    match speed {
        1..=4 => 221 * minutes / 60,
        5..=8 => ((221 * minutes / 60) as f64 * 0.9) as i32,
        9..=10 => ((221 * minutes / 60) as f64 * 0.77) as i32,
        _ => 0,
    }
}

fn is_palindrome(string: &str) -> bool {
    for (i, c) in string.char_indices() {
        if c != string.as_bytes()[string.len() - 1 - i] as char {
            return false;
        }
    }
    true
}

fn pythagorean_triplet() -> (u32, u32, u32) {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;

    for i in 1..1000_u32 {
        for j in 1..1000_u32 {
            for k in 1..1000_u32 {
                if i < j && j < k && i + j + k == 1000 && i.pow(2) + j.pow(2) == k.pow(2) {
                    a = i;
                    b = j;
                    c = k;
                    break;
                }
            }
        }
    }
    (a, b, c)
}

#[allow(dead_code)]
fn can_see_movie(age: i32, permission: bool) -> bool {
    if age >= 17 || (age >= 13 && permission) {
        return true
    }
    false
}
