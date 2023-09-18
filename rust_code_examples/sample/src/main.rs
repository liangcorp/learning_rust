fn find_longest_word(text: &str) -> &str {
    let words = text.split_whitespace();
    let mut longest_word = "";

    for word in words {
        if word.len() > longest_word.len() {
            longest_word = word;
        }
    }

    longest_word
}

fn main() {
    let text = "The customer is very important the customer will be followed by the customer";
    let longest_word = find_longest_word(text);
    println!("Longest Word: {}", longest_word);
}

