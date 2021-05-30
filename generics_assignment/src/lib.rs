extern crate num;
use num::Num;

pub trait Addition {
    fn add<T: Num>(&self, num1: T, num2: T) -> T;
}

pub struct Numbers <T: Num> {
    pub x: T,
    pub y: T,
}

impl <T: Num> Addition for Numbers <T> {
    fn add<U: Num>(&self, num1: U, num2: U) -> U {
        num1 + num2
    }
}