use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Error(error) => {
            panic!("File not found");
        }
    }

    println!("{:?}", f);
}