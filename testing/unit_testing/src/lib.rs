fn fun1() -> i32 {
    2 + 2
}

pub fn fun2() -> i32 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use super::fun1;
    use super::fun2;

    #[test]
    fn it_works() {
        assert_eq!(fun1(), 4);
        assert_eq!(fun2(), 4);
    }
}
