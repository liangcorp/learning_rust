use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let output = read();

    match output {
        Ok(fi) => println!("{:?}", fi),
        Err(e) => println!("{:?}", e),
    };
}

// Propagating error using question mark
// Question mark only returns "Err"
// Programmer's job to return "Ok"
fn read() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    match f.read_to_string(&mut s)?;

    Ok(s)
}