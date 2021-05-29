use std::io;

fn main() {
    let mut no = String::new();
    io::stdin().read_line(&mut no).expect(("Fail"));

    let no: u32 = match no.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }

    println!("{}", no);
}