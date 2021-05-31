use std::rc::RC;
use std::cell:Refcell;

fn main() {
    let value = Rc::new(Refcell::new(5));

    let a = Rc::clone(&value);

    let b = Rc::clone(&value);

    *value.borrow_mut() += 5;

    println!("a after {:?}", a);

    *value.borrow_mut() += 5;

    println!("b after {:?}", b);

    *value.borrow_mut() += 5;

    println!("a value {:?}", value);

}