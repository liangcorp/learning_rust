fn main() {
    println!(
        "The first longest word is {:?}",
        find_longest_word2("sample string words")
    );
}

#[allow(dead_code)]
fn find_longest_word2(input_string: &str) -> &str {
    let mut max = 0;
    let mut result = "";

    let str_vec: Vec<&str> = input_string
        .split_whitespace()
        .collect();

    str_vec.iter().for_each(|x| {
        if x.len() > max {
            max = x.len();
            result = x
        }
    });

    println!("address of result str {:p}", result);
    result
}

#[allow(dead_code)]
fn find_longest_word(input_string: &str) -> String {
    let mut max = 0;
    let mut result = String::new();

    let str_vec: Vec<String> = input_string
        .split_whitespace()
        .map(str::to_string)
        .collect();

    str_vec.iter().for_each(|x| {
        if x.len() > max {
            max = x.len();
            result = x.to_string()
        }
    });

    result
}
