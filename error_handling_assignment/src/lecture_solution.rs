fn document() -> String {
    let doc = "Yes";
    println!("Do you have all documents");

    if doc == "Yes" {
        println!("{}\n", doc);
        String::from("Yes")
    } else {
        println!("{}\n", doc);
        String::from("No")
    }
}

fn fees() -> String {
    let fees = "Ok";
    println!("Please submit your fees");

    if fees == "Ok" {
        println!("{}\n", fees);
        String::from("Yes")
    } else {
        println!("{}\n", fees);
        String::from("No");
    }
}

fn main() {
    println!("Welcome");
    let doc = document();

    if doc == String::from("Yes") {
        println!("Thank you for submitting documents.");
        println!("Now submit fees");

        let f == fees();
        if f == String::from("Yes") {
            println!("Thank you for submitting fees");
            println!("Admission confirmed");
        } else {
            println!("Sorry you have not submitted fees");
            println!("Admission Cancelled.\n");
            panic!("");
        }
    } else {
        println!("Sorry you have not submitted document");
            println!("Admission Cancelled.\n");
            panic!("");
    }
}