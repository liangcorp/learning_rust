use std::io;

fn find_vowel(c: &String) -> bool {
    c == "a" || c == "e" || c == "i" || c == "o" || c == "u"
}

fn calculate(a: &i32, b: &i32, op: &String) -> i32 {
    if op == "+" {
        a + b
    } else if op == "-" {
        a - b
    } else if op == "*" {
        a * b
    } else {
        a / b
    }
}

fn find_grade(grade: &i32) -> String {
    let mut result = String::new();

    if *grade >= 90 {
        result.push('A');
    } else if *grade >= 80 && *grade < 90 {
        result.push('B');
    } else if *grade >= 70 && *grade < 80 {
        result.push('C');
    } else if *grade >= 60 && *grade < 70 {
        result.push('D');
    } else {
        result.push_str("Fail");
    }

    result
}

fn main() {
    /*
        Assignment 1:
        Find a alphabet is vowel or not
    */
    let mut c = String::new();

    println!("Enter a letter:");
    io::stdin().read_line(&mut c).expect("Failed");

    c = c.trim().to_string();

    if find_vowel(&c) {
        println!("\'{}\' is vowel", c);
    } else {
        println!("\'{}\' is not vowel", c);
    }

    /*
        Assignment 2:
        Create basic calculator which support +, -, /, *
    */
    let mut a = String::new();
    let mut b = String::new();
    let mut op = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut a).expect("Failed");
    io::stdin().read_line(&mut b).expect("Failed");

    println!("Enter a math operator:");
    io::stdin().read_line(&mut op).expect("Failed");

    let a: i32 = a.trim().parse().expect("Failed");
    let b: i32 = b.trim().parse().expect("Failed");
    op = op.trim().to_string();

    let result: i32 = calculate(&a, &b, &op);
    println!("Result is {}", result);

    /*
        Assignment 3:
        Find grade from percentage
    */

    let mut grade = String::new();

    println!("Enter your grade:");
    io::stdin().read_line(&mut grade).expect("Failed");
    let grade: i32 = grade.trim().parse().expect("Failed");

    println!("{} is {}.", grade, find_grade(&grade));
}
