use std::collections::HashMap;
use std::io;

fn search_contact(name: &String, contact: &HashMap<&String, &i32>) -> i32 {
    let mut phone: i32 = 0;

    // println!("{:?}", contact);

    for (n, p) in contact {
        if *n == name {
            phone = **p;
            break;
        }
    }

    phone
}

fn main() {
    let mut name = Vec::new();
    let mut phone_no = Vec::new();

    name.push(String::from("Bob"));
    name.push(String::from("Alice"));

    phone_no.push(123456);
    phone_no.push(654321);

    let contact: HashMap<_, _> = name.iter().zip(phone_no.iter()).collect();

    let mut search_name = String::new();

    println!("Enter a name to search:");
    io::stdin().read_line(&mut search_name).expect("Failed");

    search_name = search_name.trim().to_string();

    let phone_no = search_contact(&search_name, &contact);

    if phone_no == 0 {
        println!("Contact not found");
    } else {
        println!("{}'s phone number is {}", search_name, phone_no);
    }
}
