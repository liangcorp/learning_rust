use std::io;

fn submit_doc(answer: String) -> Result<String, String> {
    if answer == String::from("yes") {
        Ok(String::from("submitted"))
    } else {
        Err(String::from("not submitted"))
    }
}

fn paid_fee(answer: String) -> Result<String, String> {
    if answer == String::from("yes") {
        Ok(String::from("paid"))
    } else {
        Err(String::from("not paid"))
    }
}

fn main() {
    let mut answer = String::new();

    println!("Have you submitted your document?");

    let mut answer = match io::stdin().read_line(&mut answer) {
        Ok(_) => answer,
        Err(error) => {
            panic!("{}", error);
        }
    };

    answer = answer.trim().to_string();

    match submit_doc(answer.to_string()) {
        Ok(s) => {
            println!("Document {}", s);
        },
        Err(e) => {
            panic!("Document {}", e);
        },
    };

    println!("Have you submitted your fees?");
    answer.clear();
    answer = match io::stdin().read_line(&mut answer) {
        Ok(_) => answer,
        Err(error) => {
            panic!("{}", error);
        }
    };

    answer = answer.trim().to_string();

    match paid_fee(answer.to_string()) {
        Ok(s) => {
            println!("Fee {}", s);
        },
        Err(e) => {
            panic!("Fee {}", e);
        },
    };
}
