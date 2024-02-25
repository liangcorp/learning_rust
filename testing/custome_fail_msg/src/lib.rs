fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rob");
        assert!(result.contains("Rob"),
            "Greeting did not contain name, value was {}", result);
    }
}
