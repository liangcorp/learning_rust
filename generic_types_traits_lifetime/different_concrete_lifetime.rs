fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* Compilation Error */
fn main() {
    let s1 = String::from("Hello");

    // scope of main
    let result;


    {   // begin s2 scope
        let s2 = String::from("Bye");
        // what if s2 is the longest: result -> s2
        result = longest(&s1, &s2);
        println!("{}", result);
    }   // end s2 scope

    // result -> ?

}
/* Compilation Error */
fn main() {
    let s1 = String::from("Hello");

     {
        let result; // Create result
        let s2 = String::from("Bye");   // create s2

        // what if s2 is the longest: result -> s2
        result = longest(&s1, &s2);

        println!("{}", result);
    }   // drop s2 then drop result. result -> ?
}

/* Compiles fine */
fn main() {
    let s1 = String::from("Hello");

     {
        let s2 = String::from("Bye");   // create s2

        // what if s2 is the longest:
        let result = longest(&s1, &s2); // create result -> s2

        println!("{}", result);
        // result -> s2
    }   // drop result then drop s2. No dangling reference
}