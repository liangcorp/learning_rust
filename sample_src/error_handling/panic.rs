fn main() {
    let choice = "buy";

    println!("What's your choice?");

    if choice == "buy" {
        println!("Thank you");
    }
    else {
        panic!("Please buy");
    }
}