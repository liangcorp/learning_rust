struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}",
        self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{data: String::from("my stuff")};
    c.drop();   // Not allowed because of "double free" error


    drop(c);    // works fine
    let d = CustomSmartPointer{data: String::from("other stuff")};

    println!("CustomSmartPointer Created");
}   // c is dropped again by automatic clean up