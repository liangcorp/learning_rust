fn prints_and_returns(a: i32) -> i32 {
    println!("I got the value {}", a);
    a
}

#[cfg(test)]
mod tests {
    use super::prints_and_returns;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns(10);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns(10);
        assert_eq!(5, value);
    }
}
